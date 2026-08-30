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

##VARIABLE##
#ZERO LEVEL#
ADB=/data/adb
#ONE LEVEL#
FSEEMODDIR=${ADB}/modules/fs_enhancer_extreme
FSEEDIR=${ADB}/fs_enhancer_extreme
#TWO LEVEL#
OLDLOG=${FSEEDIR}/log.old
LOGDIR=${FSEEDIR}/log
FSEELOG=${LOGDIR}/log.log
INTERCEPT=${FSEEDIR}/intercept
#OTHER#
LOG_TAG='<Undefined>'
case "${0##*/}" in
    'post-fs-data.sh')
        LOG_TAG='<post-fs-data>'
        ;;
    '.fsee_state.sh')
        LOG_TAG='<service.d>'
        ;;
    'service.sh')
        LOG_TAG='<service>'
        ;;
esac
##END##

##FUNCTIONS##
fseec() {
    ${FSEEMODDIR}/bin/fseec ${@}
}
output() {
    echo "`date '+%m-%d %H:%M:%S.%3N'`  ${$}  ${$} ${1} [FSEE]  : ${LOG_TAG} ${2}" >> "${FSEELOG}"
}
logI() {
    output 'I' "${*}"
}
logW() {
    output 'W' "${*}"
}
logE() {
    output 'E' "${*}"
}
first_initial() {
    rm -rf "${OLDLOG}"
    mv -f "${LOGDIR}" "${OLDLOG}"
    mkdir -p "${LOGDIR}"
    touch "${FSEELOG}"
    logI '完成日志轮换'
    rm -f "${INTERCEPT}"
}
last_initial() {
    [ -x "${ADB}/service.d/.fsee_state.sh" ] || {
        logI '配置描述文件刷新脚本'
        mkdir -p "${ADB}/service.d"
        cp -f "${FSEEMODDIR}/script/state.sh" "${ADB}/service.d/.fsee_state.sh"
        chmod +x "${ADB}/service.d/.fsee_state.sh"
    }
    action_disable() {
        mv -f "${FSEEMODDIR}/action.sh" "${FSEEMODDIR}/script/action.sh" > /dev/null 2>&1
    }
    if fseec envcheck
    then
        logI '环境正常'
        mv -f "${FSEEMODDIR}/other/webroot" "${FSEEMODDIR}/webroot" > /dev/null 2>&1
        if [ ${APATCH} ] || [ ${KSU} ]
        then
            action_disable
        else
            mv -f "${FSEEMODDIR}/script/action.sh" "${FSEEMODDIR}/action.sh" > /dev/null 2>&1
        fi
    else
        logE '环境异常'
        touch "${INTERCEPT}"
        mv -f "${FSEEMODDIR}/webroot" "${FSEEMODDIR}/other/webroot" > /dev/null 2>&1
        action_disable
    fi
}
intercept() {
    [ -f "${INTERCEPT}" ] && {
        logE '拦截执行'
        exit
    }
}
initwait() {
    until [ `getprop sys.boot_completed` -eq 1 ]
    do
        sleep 1s
    done
}
invoke() {
    logI "fseec(\"${*}\")"
    if fseec ${@}
    then
        logI "Ok(\"${*}\")"
    else
        logW "Err(\"${*}\")"
    fi
}
##END##