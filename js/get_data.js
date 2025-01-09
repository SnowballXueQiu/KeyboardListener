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
      "event_type": "keyboard_press",
      "content": "Ctrl",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355201,
      "event_type": "keyboard_press",
      "content": "C",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355202,
      "event_type": "keyboard_release",
      "content": "C",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355203,
      "event_type": "keyboard_release",
      "content": "Ctrl",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355204,
      "event_type": "clipboard_copy",
      "content": "这是一段很长的文本，用来测试复制功能和文本换行效果。The quick brown fox jumps over the lazy dog.",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355300,
      "event_type": "keyboard_press",
      "content": "Shift",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355301,
      "event_type": "keyboard_press",
      "content": "Tab",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355302,
      "event_type": "keyboard_release",
      "content": "Tab",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355303,
      "event_type": "keyboard_release",
      "content": "Shift",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355400,
      "event_type": "clipboard_copy",
      "content": "用户名：admin 密码：******",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355500,
      "event_type": "keyboard_press",
      "content": "Alt",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355501,
      "event_type": "keyboard_press",
      "content": "F4",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355502,
      "event_type": "keyboard_release",
      "content": "F4",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355503,
      "event_type": "keyboard_release",
      "content": "Alt",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355600,
      "event_type": "clipboard_copy",
      "content": "https://example.com/very/long/url/path/to/some/resource?param1=value1&param2=value2",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355700,
      "event_type": "keyboard_press",
      "content": "Win",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355701,
      "event_type": "keyboard_press",
      "content": "D",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355702,
      "event_type": "keyboard_release",
      "content": "D",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355703,
      "event_type": "keyboard_release",
      "content": "Win",
      "timezone": "UTC+8"
    },
    {
      "time": 1638355800,
      "event_type": "clipboard_copy",
      "content": "多行文本测试：\n第一行内容\n第二行内容\n第三行内容",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356000,
      "event_type": "keyboard_press",
      "content": "Ctrl",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356001,
      "event_type": "keyboard_press",
      "content": "V",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356002,
      "event_type": "keyboard_release",
      "content": "V",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356003,
      "event_type": "keyboard_release",
      "content": "Ctrl",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356100,
      "event_type": "clipboard_copy",
      "content": "SELECT * FROM users WHERE username = 'admin' AND password = 'test123'",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356200,
      "event_type": "keyboard_press",
      "content": "Ctrl",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356201,
      "event_type": "keyboard_press",
      "content": "A",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356202,
      "event_type": "keyboard_release",
      "content": "A",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356203,
      "event_type": "keyboard_release",
      "content": "Ctrl",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356300,
      "event_type": "clipboard_copy",
      "content": "const handleSubmit = async (e) => { e.preventDefault(); await fetch('/api/submit', { method: 'POST', body: JSON.stringify(formData) }); };",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356400,
      "event_type": "keyboard_press",
      "content": "Win",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356401,
      "event_type": "keyboard_press",
      "content": "R",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356402,
      "event_type": "keyboard_release",
      "content": "R",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356403,
      "event_type": "keyboard_release",
      "content": "Win",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356500,
      "event_type": "clipboard_copy",
      "content": "npm install react@latest react-dom@latest @types/react@latest @types/react-dom@latest",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356600,
      "event_type": "keyboard_press",
      "content": "Shift",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356601,
      "event_type": "keyboard_press",
      "content": "Enter",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356602,
      "event_type": "keyboard_release",
      "content": "Enter",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356603,
      "event_type": "keyboard_release",
      "content": "Shift",
      "timezone": "UTC+8"
    },
    {
      "time": 1638356700,
      "event_type": "clipboard_copy",
      "content": "docker run -d --name postgres -e POSTGRES_PASSWORD=mysecretpassword -p 5432:5432 postgres:latest",
      "timezone": "UTC+8"
    }
  ],
  "b6d7c3a1": [
    {
      "time": 1638357000,
      "event_type": "keyboard_press",
      "content": "Shift",
      "timezone": "UTC+8"
    },
    {
      "time": 1638358800,
      "event_type": "clipboard_copy",
      "content": "多行文本测试：\n第一行内容\n第二行内容\n第三行内容",
      "timezone": "UTC+8"
    }
  ],
  "a8e7b9f4": [
    {
      "time": 1638360600,
      "event_type": "clipboard_copy",
      "content": "特殊字符测试：!@#$%^&*()_+{}|:\"<>?[]\\;',./`~    以及一些空格和制表符\t还有一些特殊符号：©®™",
      "timezone": "UTC+8"
    },
    {
      "time": 1638361500,
      "event_type": "keyboard_press",
      "content": "Alt",
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