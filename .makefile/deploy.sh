#!/bin/sh

mkdir -p /root/.ssh
cp /ssh/* /root/.ssh/
chmod 600 /root/.ssh/id_*
printf "StrictHostKeyChecking=no\nControlMaster auto\nControlPath ~/.ssh/control:%%h:%%p:%%r\nControlPersist 600" >> /etc/ssh/ssh_config
ssh "${NIGHTPI_DEPLOY_IP}" mkdir -p "${NIGHTPI_DEPLOY_DIR}"
rsync -tvuE $1/nightpi "${NIGHTPI_DEPLOY_IP}:${NIGHTPI_DEPLOY_DIR}"
ssh ${NIGHTPI_DEPLOY_IP} chmod +x "${NIGHTPI_DEPLOY_DIR}/nightpi"