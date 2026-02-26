# ccpclean

[English](./README_EN.md)

一个终端工具，用于查找并清理残留的本地开发服务进程 —— 比如在 Claude Code、VS Code 或终端中启动的 Node、Python、Deno、Bun 等服务，在会话结束后仍然在后台运行。

使用 Rust 构建，跨平台支持（Windows、macOS、Linux），提供交互式 TUI 界面。

## 解决什么问题？

你在 Claude Code 或终端里运行了 `npm run dev` 或 `python manage.py runserver`，关闭会话后，进程仍然在后台运行，占着 3000、5173、8000 这些端口……

`ccpclean` 会扫描系统中这些残留的服务进程，根据多项特征评估它们是"遗留开发服务"的可能性，并通过交互式界面让你选择性地关闭它们。

## 安装

```bash
cargo install ccpclean
```

或者从源码构建：

```bash
git clone https://github.com/yeung66/ccpclean.git
cd ccpclean
cargo install --path .
```

## 使用方式

```bash
# 启动交互式 TUI（默认严格模式，仅显示开发运行时进程）
ccpclean

# 显示所有监听本地端口的进程（宽松模式）
ccpclean --all

# 按端口过滤
ccpclean --port 3000

# 非交互模式（适用于脚本或快速查看）
ccpclean --no-tui
```

## TUI 界面

### 列表视图（默认）

通过复选框多选进程，批量终止。

```
 ccpclean  [Strict: dev runtimes only]  Tab=detail view  F=switch filter
+------+-------+----------+------------+-------+------------+
|      | PID   | Name     | Ports      | Score | Command    |
+------+-------+----------+------------+-------+------------+
| [x]  | 12345 | node     | 3000, 3001 | ****- | server.js  |
| [ ]  | 23456 | python   | 8000       | ***-- | manage.py  |
| [ ]  | 34567 | node     | 5173       | ***** | vite.js    |
+------+-------+----------+------------+-------+------------+
 Space=select  A=all  Enter=kill  F=switch filter  Tab=detail  Q=quit
```

### 详情视图（按 Tab 切换）

逐个浏览进程的完整信息：PID、命令行、运行时长、内存占用、父进程以及可信度评分。

```
 Process List          Process Detail
+--------------------+----------------------------------+
| > node    :3000    | PID:     12345                   |
|   python  :8000    | Name:    node                    |
|   node    :5173    | Ports:   3000, 3001              |
|                    | Command: node server.js --watch  |
|                    | Started: 2h 13m ago              |
|                    | Memory:  87.4 MB                 |
|                    | Parent:  bash (PID 11111)        |
|                    | Score:   ****- High              |
+--------------------+----------------------------------+
```

## 快捷键

| 按键 | 功能 |
|------|------|
| `↑` / `↓` 或 `j` / `k` | 上下移动光标 |
| `Space` | 选中 / 取消选中当前进程 |
| `A` | 全选 / 取消全选 |
| `Enter` | 终止选中的进程（列表视图）或当前进程（详情视图） |
| `Tab` | 在列表视图和详情视图之间切换 |
| `F` | 切换过滤模式：**严格模式**（仅开发运行时） ↔ **宽松模式**（所有监听进程） |
| `Q` / `Esc` | 退出 |

## 过滤模式

| 模式 | 显示内容 |
|------|----------|
| **严格模式**（默认） | 仅显示开发运行时进程：`node`、`python`、`deno`、`bun`、`ruby`、`java` 等 |
| **宽松模式**（`--all` 或按 `F`） | 显示所有监听本地端口的进程，包括系统服务 |

## 可信度评分

每个进程会获得 0–100 的评分，表示它是"遗留开发服务"的可能性：

| 条件 | 加分 |
|------|------|
| 进程名是开发运行时（node、python、deno、bun、ruby……） | +30 |
| 监听端口在 1024–9999 之间 | +20 |
| 命令行包含开发关键词（server、dev、start、watch……） | +20 |
| 父进程是 shell（bash、zsh、sh、pwsh、claude……） | +20 |
| 运行时间超过 30 分钟 | +10 |

评分以圆点展示：`****-` = 80/100。

## 命令行参数

```
ccpclean [OPTIONS]

选项：
  -a, --all          宽松模式：显示所有监听本地端口的进程
  -p, --port <PORT>  按指定端口过滤
      --no-tui       非交互模式：输出列表后退出
  -h, --help         显示帮助
  -V, --version      显示版本
```

## 环境要求

- Rust 1.70+（编译需要）
- Windows 下可能需要管理员权限才能查看所有端口映射关系或终止某些进程

## License

MIT
