use rustgis_plugin::{Kind, Request, ToolOutput, Value, geojson, host, json};

fn handle(req: Request) -> Result<Value, String> {
    match (req.kind, req.id.as_str()) {
        (Kind::Command, "layer-report") => {
            let n = host::layers()?.as_array().map_or(0, Vec::len);
            host::notify(&format!("{n} layers"));
            Ok(json!({"layers": n}))
        }
        (Kind::Tool, "bounding-boxes") => {
            let boxes: Vec<Value> = req
                .features("input")?
                .iter()
                .filter_map(|f| {
                    let b = geojson::bbox(&geojson::positions(&f["geometry"]))?;
                    Some(geojson::feature(
                        geojson::polygon(&[[b[0], b[1]], [b[2], b[1]], [b[2], b[3]], [b[0], b[3]]]),
                        json!({}),
                    ))
                })
                .collect();
            Ok(ToolOutput::new()
                .layer("Boxes", geojson::collection(boxes))
                .into())
        }
        (_, other) => Err(format!("unknown '{other}'")),
    }
}

rustgis_plugin::export!(handle);
