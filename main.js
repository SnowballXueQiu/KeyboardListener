import { getDeviceList, getDeviceLogs } from './get_data.js';

// 渲染设备列表
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
}

// 显示设备信息
async function showDeviceInfo(deviceId, deviceName, e) {
  // 更新设备项的激活状态
  document.querySelectorAll('.device-item').forEach(item => {
    item.classList.remove('active');
  });
  e.target.classList.add('active');

  const deviceInfoElement = document.getElementById('deviceInfo');
  const events = await getDeviceLogs(deviceId);

  let html = `<h2>${deviceName} - ${deviceId} 的事件信息</h2>`;

  if (events.length === 0) {
    html += '<p>暂无事件记录</p>';
  } else {
    events.forEach(event => {
      const date = new Date(event.time * 1000);
      html += `
                <div class="event-item">
                    <p><strong>时间：</strong>${date.toLocaleString()} (${event.timezone})</p>
                    <p><strong>事件类型：</strong>${event.event_type}</p>
                    <p><strong>内容：</strong>${event.content}</p>
                </div>
            `;
    });
  }

  deviceInfoElement.innerHTML = html;
}

// 初始化页面
window.onload = function () {
  renderDeviceList();
}; 