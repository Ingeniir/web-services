use rustgis_plugin::{Kind, Request, Value, host, json};

mod catalog;
use catalog::{CATALOG, Service, Source};

fn handle(req: Request) -> Result<Value, String> {
    if let Some(s) = CATALOG.iter().find(|s| s.id == req.id) {
        return match s.source {
            Source::Tiles => add_tiles(s, req.bool("add") == Some(true)),
            Source::Wfs => add_wfs(s),
        };
    }
    match (req.kind, req.id.as_str()) {
        (Kind::Command, "clear") => clear(),
        (_, other) => Err(format!("unknown '{other}'")),
    }
}

/// Même logique que choose() de Basemaps : un overlay déjà présent est retiré.
fn add_tiles(s: &Service, add: bool) -> Result<Value, String> {
    if s.overlay && list_tiles()?.iter().any(|t| t["id"] == s.id) {
        return host::run("map.removeTiles", json!({ "id": s.id }));
    }
    host::run(
        "map.addTiles",
        json!({
            "url": s.url,
            "kind": "raster", // WMS compris : c'est {bbox-epsg-3857} qui en fait un WMS
            "name": s.name,
            "attribution": s.attribution,
            "maxZoom": s.max_zoom,
            "overlay": s.overlay,
            "id": s.id,
            "replace": !s.overlay && !add,
        }),
    )
}

fn add_wfs(s: &Service) -> Result<Value, String> {
    let url = s.url.replace("{bbox}", &view_bbox()?);
    let r = host::run("layer.addUrl", json!({ "url": url }))?;
    if r["pending"] == true {
        host::notify(&format!("{} : téléchargement en cours…", s.name));
    }
    Ok(r)
}

/// Accepte un tableau direct ou {"tiles": [...]}. Aligne sur le helper de Basemaps.
fn list_tiles() -> Result<Vec<Value>, String> {
    let v = host::run("map.tiles", json!({}))?;
    v.as_array()
        .or_else(|| v["tiles"].as_array())
        .cloned()
        .ok_or_else(|| format!("map.tiles : réponse inattendue {v}"))
}

/// Retire uniquement nos couches, par id : pas de souci d'index qui se décalent.
fn clear() -> Result<Value, String> {
    let mut n = 0;
    for t in list_tiles()? {
        if let Some(id) = t["id"]
            .as_str()
            .filter(|id| CATALOG.iter().any(|s| s.id == *id))
        {
            host::run("map.removeTiles", json!({ "id": id }))?;
            n += 1;
        }
    }
    Ok(json!({ "removed": n }))
}

/// Emprise approximative de la vue en lon/lat. host.view ne donne que le centre
/// et l'échelle : on suppose une vue d'environ 1600 × 1000 px et un pixel OGC de 0,28 mm.
fn view_bbox() -> Result<String, String> {
    let v = host::run("host.view", json!({}))?;
    let (Some(lon), Some(lat), Some(scale)) = (
        v["center"][0].as_f64(),
        v["center"][1].as_f64(),
        v["scale"].as_f64(),
    ) else {
        return Err(format!("host.view : réponse inattendue {v}"));
    };
    let m_px = scale * 0.000_28;
    let dlon = 800.0 * m_px / (111_320.0 * lat.to_radians().cos().max(0.01));
    let dlat = 500.0 * m_px / 110_540.0;
    Ok(format!(
        "{:.6},{:.6},{:.6},{:.6}",
        (lon - dlon).max(-180.0),
        (lat - dlat).max(-90.0),
        (lon + dlon).min(180.0),
        (lat + dlat).min(90.0)
    ))
}

#[cfg(test)]
mod tests {
    use super::catalog::{CATALOG, Source};

    #[test]
    fn urls_are_well_formed() {
        let mut ids = std::collections::HashSet::new();
        for s in CATALOG {
            assert!(ids.insert(s.id), "id en double : {}", s.id);
            assert!(s.url.starts_with("https://"), "{} : https", s.id);
            assert!(
                !s.url.contains(char::is_whitespace),
                "{} : espace dans l'URL",
                s.id
            );
            let u = s.url.to_ascii_uppercase();
            match s.source {
                Source::Tiles => {
                    let xyz = ["{Z}", "{X}", "{Y}"].iter().all(|p| u.contains(p));
                    let wms = u.contains("{BBOX-EPSG-3857}")
                        && (u.contains("CRS=EPSG:3857") || u.contains("SRS=EPSG:3857"))
                        && u.contains("WIDTH=256")
                        && u.contains("HEIGHT=256");
                    assert!(xyz ^ wms, "{} : ni XYZ/WMTS ni WMS valide", s.id);
                }
                Source::Wfs => {
                    assert!(u.contains("REQUEST=GETFEATURE"), "{}", s.id);
                    assert!(
                        u.contains("OUTPUTFORMAT=APPLICATION/JSON"),
                        "{} : GeoJSON",
                        s.id
                    );
                    assert!(
                        u.contains("COUNT=") || u.contains("MAXFEATURES="),
                        "{} : limite",
                        s.id
                    );
                }
            }
        }
    }
}

rustgis_plugin::export!(handle);
