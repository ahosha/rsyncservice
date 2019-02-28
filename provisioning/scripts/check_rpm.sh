#!/usr/bin/env bash

#to do updating the rpm's
RPM_DIR=/opt/rpm
PKG_DIR=/etc/ha/anchor

if [ -d "$PKG_DIR" ]; then
rm -r ${RPM_DIR}/*

(cd ${RPM_DIR}); echo "Updating RPM"  | rpm -Uvh anchor*

#else
#
#(cd ${RPM_DIR}); echo "Installing RPM"  | rpm -ivh anchor*

fi

#ls -t b2* | head -1
