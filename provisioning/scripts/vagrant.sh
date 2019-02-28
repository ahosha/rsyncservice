#!/usr/bin/env bash


INSTALL_USER=vagrant
USER_HOME=/home/${INSTALL_USER}

WORKSPACE=${USER_HOME}/rust/src/q1git.canlab.ibm.com/ha/anchor

IP_FILE=${WORKSPACE}/provisioning/vagrant.env

SSH_PUBLIC_KEY='ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAACAQCx5UaezPQ/3an2SHYVnKYyZZWAJmu9iUPH0MUhUBM77L17+UG8ISNqvyRexcp8C7Q633BCi+7MMuDCF7zMXfgc37sKABIEN2yUY0sc1pEkiG+zH1mWw+/Fs4sPwpx73EjS5zIgQHiynNOZ6rA3+iay3ObGWGnKddVAzuAWLeFnk8P4fy2jjJACmrcByPOpvjHQEtJBQzASVGxB/TxFLE9A6zrXTys/y6wapQeoqgSmAhR8bPb/7AUouN1ZraYn8dh2CnsoAcHg5opGFsnApjviZE/7ZdxP6SJPrbWKDZ6JVOdfl9oDbaRx84KgKOu88hKAhK3gsgKoxl5sH+FwKqGDJe0fL6vHAeLL6A9h7SJXy29qZ6j3FTUqj/tT/+EHa3aKn9YE0IAJQyOGdBwYJBs1snFvFcnae1STpbiVAnnsH6w1AeX3NGa8BW1l34ejVT1ACi0vSwC2qye83lH/HcDZ4/OPf0l1PpCLFUd4V/wCD+uqaDNBSMT15Zayuz0dHo2tdFsrWTdTGz2/OoiGO90ZRoi9nbekui90xGMI4MFE2rJDRayFx03tsA3JhEY73OOml4QtuFXKl8wAfm8Se4k6wWeDQ9B05fVYLXvBsrCgklRZNZIY5lQcs4rl34+X7B+K5xmfBIFra3MsbqaFW41VSF+YN6pWr3oVfBDaGnq5Uw== permanent public key'
SSH_KEY_NAME=${SSH_PUBLIC_KEY#* *}

DEV_RPMS="vim-enhanced wget java jq"

#Check if the rust lang is installed or not
installed_and_linked()
{
    local path="/usr/bin/$1"
    test -f ${path} && ${path} version 2>&1 > /dev/null
}

#Install rust
install_rust()
{
    if installed_and_linked rust ; then
      echo "Rust is already installed"
     else
       echo "Installing Rust"
       echo 'curl https://sh.rustup.rs -sSf | sh -s -- -y;' | su vagrant
    fi

}

#Install tools on vagrant vm boxes
install_dev_rpms()
{
    if ! rpm -q epel-release ; then
        echo "Installing CentOS epel repository"
        yum install -y epel-release || fatal "Failed to install CentOS epel repository"
    fi

    if ! rpm -q ${DEV_RPMS} ; then
        echo "Installing development RPMs"
        yum install -y ${DEV_RPMS} || fatal "Failed to install development RPMs"
    fi
}


add_authorized_key()
{
    local ssh_dir=$1/.ssh
    local authorized_keys=${ssh_dir}/authorized_keys
    local id_rsa=${ssh_dir}/id_rsa
    local id_rsapub=${ssh_dir}/id_rsa.pub

    local user=$2
    [[ -d ${ssh_dir} ]] || mkdir -p "${ssh_dir}"
    [[ -f ${authorized_keys} ]] || touch "${authorized_keys}"
    [[ -f ${id_rsa} ]] || touch "${authorized_keys}"
    [[ -f ${id_rsapub} ]] || touch "${id_rsapub}"
    if ! grep -q "${SSH_KEY_NAME}" "${authorized_keys}" ; then
        echo "Installing public key in ${authorized_keys}"
        echo "${SSH_PUBLIC_KEY}" >> ${authorized_keys}
    fi
     cp "${WORKSPACE}/provisioning/.keys/id_rsa" "${id_rsa}"
     cp "${WORKSPACE}/provisioning/.keys/id_rsa.pub" "${id_rsapub}"
    chmod 600 ${id_rsa}
    chown -R ${user} ${ssh_dir}
    chmod 700 ${ssh_dir}
    chmod 600 ${authorized_keys}

}

running_service()
{
    local systemd_file=/lib/systemd/system/rsyncservice.service
    local rust_service=${WORKSPACE}/systemd/rsyncservice.service


    [[ -f ${systemd_file} ]] || touch "${systemd_file}"
     cp "${rust_service}" "${systemd_file}"

}
configure_ssh()
{
    add_authorized_key '/root' 'root'
    add_authorized_key ${USER_HOME} ${INSTALL_USER}
}

#Config the workspace on the VM boxes
configure_workspace() {
    test -d '/entities' || ln -s "${WORKSPACE}/entities/src" '/'
    test -d '/conf' || ln -s "${WORKSPACE}/conf" '/'
    test -d '/conf.d' || ln -s "${WORKSPACE}/conf.d" '/'
    echo 'export GRADLE_USER_HOME=/home/vagrant/.gradle' >> /etc/profile.d/gradle.sh
}

#creates a vagrant env file
create_env_file() {
    echo "localhost" > ${IP_FILE}
}

#------------#
#--  MAIN  --#
#------------#

install_dev_rpms
install_rust
configure_workspace
configure_ssh
running_service
create_env_file
