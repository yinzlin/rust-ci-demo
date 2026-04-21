---
trigger: always_on
---
# Rust全栈开发规则：
1. 编码：rustfmt自动格式化，clippy零警告，禁用无必要unwrap/unsafe，代码精简
2. 语言：标识符英文，注释/日志/接口提示中文，自文档化代码；使用中文环境进行交互
3. 技术栈：我熟练的（Axum、Dioxus、Bevy、U3D、PG/SQLite/Redis/MongoDB、Docker、Nginx）、六边形架构
4. 命名：类型/组件大驼峰，变量/路由/数据库蛇形，常量大写蛇形
5. 开发：共享类型→后端→前端→3D集成，Bevy遵循ECS，禁止硬编码
6. 存储：连接池统一管理，按场景使用对应数据库
7. 构建：后端release，Dioxus用trunk，Docker+Nginx部署
8. 质量：提交前三检通过，核心代码高覆盖率，公共API中文文档
9. 版本：edition = "2024",resolver = "3",禁用过期语法
10. 依赖：在根Cargo.toml只引入最新版本号，feature功能在子Cargo.toml选择开启
11. 类型：所有类型都必须有注释，注释中包含类型的作用、字段的含义、字段的取值范围等
12. 工具：在使用nushell的时候优先使用nushell语法规则，bash语法规则做参考