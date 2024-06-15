# Punchme

自动签到

| 环境变量       | 说明                                | 必须 |
| -------------- | ----------------------------------- | ---- |
| SCHEDULE_RULE  | 开始签到的时间，默认`0 0 8 * * * *` | 否   |
| SMTP_SERVICE   | 邮件服务器，`smtp.qq.com`           | 否   |
| SMTP_USERNAME  | 用户，`smtp_username`               | 否   |
| SMTP_PASSWORD  | 密码，`smtp_password`               | 否   |
| EMAIL_RECEIVER | 通知邮箱                            | 否   |

## HiFiNi

HiFiNi 每日签到

| 环境变量      | 说明   | 必须 |
| ------------- | ------ | ---- |
| HIFINI_COOKIE | cookie | 是   |
