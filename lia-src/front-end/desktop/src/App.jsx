import React, { useState, useEffect } from 'react';
import FolderSelector from './components/FolderSelector';
import CommandList from './components/CommandList';
import CommandRunner from './components/CommandRunner';
import LogTerminal from './components/LogTerminal';
import AddCommand from './components/AddCommand';

function App() {
  const [selectedFolder, setSelectedFolder] = useState('');
  const [showTerminal, setShowTerminal] = useState(false);
  const [logs, setLogs] = useState([]);

  const addLog = (log) => {
    setLogs((prevLogs) => [...prevLogs, log]);
  };

  return (
    <div className="min-h-screen bg-gray-100">
      <header className="bg-blue-600 text-white p-4">
        <h1 className="text-2xl">Linux Assistant Desktop App</h1>
      </header>
      <main className="p-4">
        <FolderSelector selectedFolder={selectedFolder} setSelectedFolder={setSelectedFolder} />
        <div className="flex mt-4">
          <div className="w-1/2 pr-2">
            <CommandList />
          </div>
          <div className="w-1/2 pl-2">
            <AddCommand />
          </div>
        </div>
        <CommandRunner selectedFolder={selectedFolder} addLog={addLog} />
        <button
          onClick={() => setShowTerminal(!showTerminal)}
          className="mt-4 px-4 py-2 bg-green-500 text-white rounded"
        >
          {showTerminal ? 'Hide Terminal' : 'Show Terminal'}
        </button>
        {showTerminal && <LogTerminal logs={logs} />}
      </main>
    </div>
  );
}

export default App;
