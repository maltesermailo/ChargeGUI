import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<SyscallPage />} />
        <Route path="/export" element={<ExportPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
