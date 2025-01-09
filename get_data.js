import config from './config.js';

// 模拟设备列表数据
const mockDevices = [
  {
    "device_id": "f8e7d6c5",
    "device_name": "办公室电脑"
  },
  {
    "device_id": "b6d7c3a1",
    "device_name": "会议室投影"
  },
  {
    "device_id": "a8e7b9f4",
    "device_name": "笔记本电脑"
  }
];

// 模拟设备日志数据
const mockLogs = {
  "f8e7d6c5": [
    {
      "time": 1638355200,
      "event_type": "clipboard_copy",
      "content": "复制了会议纪要文本",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356100,
      "event_type": "file_open",
      "content": "打开了季度报告.xlsx",
      "timezone": "UTC+8"
    }
  ],
  "b6d7c3a1": [
    {
      "time": 1638357000,
      "event_type": "device_status",
      "content": "设备开机",
      "timezone": "UTC+8"
    },
    {
      "time": 1638358800,
      "event_type": "screen_share",
      "content": "开始投影演示",
      "timezone": "UTC+8"
    }
  ],
  "a8e7b9f4": [
    {
      "time": 1638360600,
      "event_type": "network_connect",
      "content": "连接到公司WiFi",
      "timezone": "UTC+8"
    },
    {
      "time": 1638361500,
      "event_type": "software_launch",
      "content": "启动Visual Studio Code",
      "timezone": "UTC+8"
    }
  ]
};

export async function getDeviceList() {
  // 返回模拟数据
  return Promise.resolve(mockDevices);
}

export async function getDeviceLogs(deviceId) {
  // 返回模拟数据
  return Promise.resolve(mockLogs[deviceId] || []);
} 