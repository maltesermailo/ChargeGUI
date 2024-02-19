import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";

function LoadPage() {
   async function loadFile() {
        const selected = await open({multiple: false, filters: [{name: "JSON Log", extensions: ['json']}]});

        invoke('load_file', { file: selected });
   }

  return (
    <div className='loadFile'>
        <button className='loadFileButton' onClick={loadFile}>Load File</button>
    </div>
  );
}

export default LoadPage;