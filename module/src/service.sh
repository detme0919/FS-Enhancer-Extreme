#
# This file is part of FS-Enhancer-Extreme.
#
# This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
#
# This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
# without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
# See the GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License along with this program;
# If not, see <https://www.gnu.org/licenses/>.
#
# Copyright (C) 2025-2026 XtrLumen
#

cd "${0%/*}"
source './script/util_functions.sh'
intercept

logI '启动服务'
fseec fseectl start || logE '服务启动失败'

initwait
logI '刷新目标列表'
invoke listrefresh
logI '处理冲突软件'
invoke appcheck
logI '将自定义安全补丁级别同步到 prop'
invoke spsyncprop
logI '重设引导加载程序解锁状态相关 prop 为锁定'
invoke passprop
logI '获取正确 VerifiedBootHash prop 并重设'
invoke passvbhash