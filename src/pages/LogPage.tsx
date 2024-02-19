import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { useState, useEffect } from 'react';

interface LogEntry {
    time: string
    text: string;
}

type LogEntryArray = LogEntry[] | [];

function LogPage() {
  const [log, setLog] = useState<LogEntryArray>([]);

  useEffect(() => {
    const listener = listen('log', (event) => {
      console.log("test2");
      console.log(event);

      setLog(currentLog => [...currentLog, {time: "16:00", text: "test"}]);
    });

    console.log("test");

    invoke('ready');

    return () => {
      console.log("test3");
      listener.then((unlistenFn) => unlistenFn());
    }
  }, []);

  return (
    <div className='bordered-list'>
        <div>
            {log.map((item) => (
                <div className='logEntry'>{item.time}: {item.text}</div>
            ))}
        </div>
    </div>
  );
}

export default LogPage;