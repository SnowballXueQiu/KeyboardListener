import config from './config.js';

let currentWebSocket = null;
let currentDeviceId = null;

export function closeWebSocket(newDeviceId = null) {
  if (currentWebSocket && currentWebSocket.readyState === WebSocket.OPEN) {
    try {
      console.log(`正在关闭WebSocket连接: ${currentDeviceId} -> ${newDeviceId}`);
      currentWebSocket.send(JSON.stringify({
        type: 'close',
        device_id: currentDeviceId,
        new_device_id: newDeviceId
      }));
      currentWebSocket.close(1000, 'Device switched or page closed');
    } catch (error) {
      console.error('关闭WebSocket连接时出错:', error);
    } finally {
      currentWebSocket = null;
      currentDeviceId = null;
    }
  }
}

export function connectWebSocket(deviceId, onMessageCallback) {
  const wsUrl = `${config.WS_BASE_URL}/ws/${deviceId}`;
  const ws = new WebSocket(wsUrl);

  currentWebSocket = ws;

  ws.onopen = () => {
    console.log(`WebSocket连接已建立: ${deviceId}`);
    currentDeviceId = deviceId;
  };

  ws.onmessage = (event) => {
    const newLog = JSON.parse(event.data);
    if (onMessageCallback) {
      onMessageCallback(newLog);
    }
  };

  ws.onclose = (event) => {
    console.log(`WebSocket连接已关闭: ${deviceId}, code: ${event.code}, reason: ${event.reason}`);
    if (currentDeviceId === deviceId) {
      currentWebSocket = null;
      currentDeviceId = null;
    }
  };

  ws.onerror = (error) => {
    console.error(`WebSocket错误 (${deviceId}):`, error);
    if (currentDeviceId === deviceId) {
      currentWebSocket = null;
      currentDeviceId = null;
    }
  };
}