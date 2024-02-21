import { save } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import { BrowserRouter, Routes, Route, useNavigate } from 'react-router-dom';

function ExportPage() {
  const navigate = useNavigate();

  async function doExport() {
    const selected = await save({filters: [{name: "YAML File", extensions: ['yaml']}]});

    invoke('export_file', { file: selected });
  }

  function back() {
    navigate("/syscalls");
  }

  return (
    <div>
      <button onClick={back}>Back</button>
      <button onClick={doExport}>Export</button>
    </div>
  );
}

export default ExportPage;