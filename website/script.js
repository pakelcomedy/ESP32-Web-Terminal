// script.js - IoT Themed Web Terminal & File Explorer

const API_FILES = '/api/files';
const API_TERM = '/api/term';

// -------- File Explorer Logic --------
const fileListEl = document.getElementById('file-list');
const uploadBtn = document.getElementById('btn-upload');
const refreshBtn = document.getElementById('btn-refresh');
const fileInput = document.getElementById('file-input');

// Fetch and render file list
async function fetchFiles() {
  fileListEl.innerHTML = '<li>Loading...</li>';
  try {
    const res = await fetch(API_FILES);
    const { files } = await res.json();
    fileListEl.innerHTML = files.map(name => 
      `<li>
        <span>${name}</span>
        <div>
          <button onclick="downloadFile('${name}')">⬇️</button>
          <button onclick="deleteFile('${name}')">🗑️</button>
        </div>
      </li>`
    ).join('');
  } catch (err) {
    fileListEl.innerHTML = '<li>Error loading files</li>';
    console.error(err);
  }
}

// Download file
function downloadFile(name) {
  window.location = `${API_FILES}/${encodeURIComponent(name)}`;
}

// Delete file
async function deleteFile(name) {
  if (!confirm(`Delete ${name}?`)) return;
  await fetch(`${API_FILES}/${encodeURIComponent(name)}`, { method: 'DELETE' });
  fetchFiles();
}

// Upload file
uploadBtn.addEventListener('click', () => fileInput.click());
fileInput.addEventListener('change', async () => {
  const file = fileInput.files[0];
  if (!file) return;
  const data = await file.arrayBuffer();
  await fetch(`${API_FILES}/upload`, {
    method: 'POST',
    headers: { 'X-Filename': file.name },
    body: data
  });
  fileInput.value = '';
  fetchFiles();
});

refreshBtn.addEventListener('click', fetchFiles);
// Initial load
fetchFiles();

// -------- Web Terminal Logic --------
const termEl = document.getElementById('term');
const cmdInput = document.getElementById('cmd-input');
let ws;

function initWebSocket() {
  ws = new WebSocket(`ws://${location.host}${API_TERM}`);
  ws.binaryType = 'arraybuffer';

  ws.onopen = () => appendLine('--- Connected to ESP32 ---');
  ws.onmessage = (e) => {
    const data = new TextDecoder().decode(e.data);
    appendLine(data.trim());
    termEl.scrollTop = termEl.scrollHeight;
  };
  ws.onclose = () => appendLine('*** Disconnected ***');
  ws.onerror = (e) => console.error('WebSocket error', e);
}

cmdInput.addEventListener('keydown', e => {
  if (e.key === 'Enter' && ws && ws.readyState === WebSocket.OPEN) {
    const cmd = cmdInput.value + '\r';
    ws.send(cmd);
    appendLine(`> ${cmdInput.value}`);
    cmdInput.value = '';
  }
});

function appendLine(text) {
  termEl.innerHTML += text.replace(/\n/g, '<br>') + '<br>';
}

// Kick off terminal
initWebSocket();
