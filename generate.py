import json
from pathlib import Path

path = Path("plugin.json")
manifest = json.loads(path.read_text(encoding="utf-8"))
manifest["commands"] = commands  # générés depuis la table
manifest["tab"] = tab  # idem
# version, i18n, permissions… restent intacts
path.write_text(
    json.dumps(manifest, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
)
