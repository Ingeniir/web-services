#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    /// XYZ/WMTS ({z}/{x}/{y}) ou WMS ({bbox-epsg-3857}) -> map.addTiles
    Tiles,
    /// WFS GetFeature en GeoJSON -> layer.addUrl ; {bbox} remplacé par le plugin
    Wfs,
}

pub struct Service {
    pub id: &'static str,
    pub name: &'static str,
    pub source: Source,
    pub url: &'static str,
    pub attribution: &'static str,
    pub max_zoom: u8,
    pub overlay: bool,
}

pub const CATALOG: &[Service] = &[
    Service {
        id: "ign-ortho",
        name: "IGN Orthophotos",
        source: Source::Tiles,
        url: "https://data.geopf.fr/wmts?SERVICE=WMTS&REQUEST=GetTile&VERSION=1.0.0\
              &LAYER=ORTHOIMAGERY.ORTHOPHOTOS&STYLE=normal&TILEMATRIXSET=PM\
              &FORMAT=image/jpeg&TILEMATRIX={z}&TILEROW={y}&TILECOL={x}",
        attribution: "© IGN",
        max_zoom: 19,
        overlay: false,
    },
    Service {
        id: "ign-plan",
        name: "Plan IGN",
        source: Source::Tiles,
        url: "https://data.geopf.fr/wmts?SERVICE=WMTS&REQUEST=GetTile&VERSION=1.0.0\
              &LAYER=GEOGRAPHICALGRIDSYSTEMS.PLANIGNV2&STYLE=normal&TILEMATRIXSET=PM\
              &FORMAT=image/png&TILEMATRIX={z}&TILEROW={y}&TILECOL={x}",
        attribution: "© IGN",
        max_zoom: 19,
        overlay: false,
    },
    Service {
        id: "ign-cadastre-wms",
        name: "Cadastre (WMS)",
        source: Source::Tiles,
        url: "https://data.geopf.fr/wms-r?SERVICE=WMS&VERSION=1.3.0&REQUEST=GetMap\
              &LAYERS=CADASTRALPARCELS.PARCELLAIRE_EXPRESS&STYLES=&FORMAT=image/png\
              &TRANSPARENT=TRUE&CRS=EPSG:3857&WIDTH=256&HEIGHT=256&BBOX={bbox-epsg-3857}",
        attribution: "© IGN",
        max_zoom: 20,
        overlay: true,
    },
    Service {
        id: "ign-communes-wfs",
        name: "Communes (WFS)",
        source: Source::Wfs,
        url: "https://data.geopf.fr/wfs/ows?SERVICE=WFS&VERSION=2.0.0&REQUEST=GetFeature\
              &TYPENAMES=ADMINEXPRESS-COG-CARTO.LATEST:commune&OUTPUTFORMAT=application/json\
              &SRSNAME=EPSG:4326&COUNT=1000&BBOX={bbox},EPSG:4326",
        attribution: "© IGN",
        max_zoom: 0,
        overlay: false,
    },
    Service {
        id: "ign-batiments-wfs",
        name: "Bâtiments BD TOPO (WFS)",
        source: Source::Wfs,
        url: "https://data.geopf.fr/wfs/ows?SERVICE=WFS&VERSION=2.0.0&REQUEST=GetFeature\
              &TYPENAMES=BDTOPO_V3:batiment&OUTPUTFORMAT=application/json\
              &SRSNAME=EPSG:4326&COUNT=5000&BBOX={bbox},EPSG:4326",
        attribution: "© IGN",
        max_zoom: 0,
        overlay: false,
    },
];
