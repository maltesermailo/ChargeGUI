import { save } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";

function ExportPage() {
  async function doExport() {
    const selected = await save({filters: [{name: "JSON Log", extensions: ['json']}]});

    invoke('export_file', { file: selected });
  }

  return (
    <div>
      <button onClick={doExport}>Export</button>
    </div>
  );
}

export default ExportPage;