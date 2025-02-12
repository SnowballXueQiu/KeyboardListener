import config from './config.js';

export async function getDeviceList() {
  try {
    const response = await fetch(`${config.API_BASE_URL}/device_id_list`);
    if (!response.ok) {
      throw new Error('获取设备列表失败');
    }
    return await response.json();
  } catch (error) {
    console.error('获取设备列表出错:', error);
    return [];
  }
}

export async function getDeviceLogs(deviceId) {
  try {
    const response = await fetch(`${config.API_BASE_URL}/log/${deviceId}`);
    if (!response.ok) {
      throw new Error('获取设备日志失败');
    }
    const data = await response.json();
    return data.sort((a, b) => b.time - b.time);
  } catch (error) {
    console.error('获取设备日志出错:', error);
    return [];
  }
}
