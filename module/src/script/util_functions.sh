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
FSEECONFIG=${FSEEDIR}/config
OLDLOG=${FSEEDIR}/log.old
LOGDIR=${FSEEDIR}/log
FSEELOG=${LOGDIR}/log.log
#OTHER#
isPostFsData=false
LOG_TAG='<Undefined>'
case "${0##*/}" in
    'post-fs-data.sh')
        isPostFsData=true
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
    echo "$(date "+%m-%d %H:%M:%S.$(date +%3N)")  ${$}  ${$} ${1} [FSEE]  : ${LOG_TAG} ${2}" >> "${FSEELOG}"
}
logI() {
    output 'I' "${1}"
}
logW() {
    output 'W' "${1}"
}
logE() {
    output 'E' "${1}"
}
initwait() {
    until [ "`getprop sys.boot_completed`" -eq 1 ]
    do
        sleep 1s
    done
}
envcheck() {
    if fseec envcheck
    then
        ${isPostFsData} && {
            logI '环境正常, 继续执行'
            mv -f "${FSEEMODDIR}/.webroot" "${FSEEMODDIR}/webroot"
            if [[ ! "${APATCH}" && ! "${KSU}" ]]
            then
                mv -f "${FSEEMODDIR}/.action.sh" "${FSEEMODDIR}/action.sh" >/dev/null 2>&1
            else
                mv -f "${FSEEMODDIR}/action.sh" "${FSEEMODDIR}/.action.sh" >/dev/null 2>&1
            fi
        }
    else
        ${isPostFsData} && {
            logE '环境异常, 拦截执行'
            mv -f "${FSEEMODDIR}/webroot" "${FSEEMODDIR}/.webroot" >/dev/null 2>&1
            mv -f "${FSEEMODDIR}/action.sh" "${FSEEMODDIR}/.action.sh" >/dev/null 2>&1
        }
        exit
    fi
}
invoke() {
    if fseec ${@} >> "${FSEELOG}" 2>&1
    then
        logI "Ok(${@})"
    else
        logW "Err(${@})"
    fi
}
##END##