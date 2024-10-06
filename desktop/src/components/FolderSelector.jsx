import React from 'react';

import tauriApi from '@tauri-apps/api';
const { open } = tauriApi.dialog;

function FolderSelector({ selectedFolder, setSelectedFolder }) {
  const selectFolder = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
    });

    if (selected) {
      setSelectedFolder(selected);
    }
  };

  return (
    <div>
      <button
        onClick={selectFolder}
        className="px-4 py-2 bg-blue-500 text-white rounded"
      >
        Select Folder
      </button>
      {selectedFolder && (
        <p className="mt-2">
          Selected Folder: <strong>{selectedFolder}</strong>
        </p>
      )}
    </div>
  );
}

export default FolderSelector;
