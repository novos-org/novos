# Detect OS and Arch
OS_NAME!=   uname -s
TARGET_OS!=  uname -s | tr '[:upper:]' '[:lower:]'
TARGET_ARCH!= uname -m

# SunOS specific flags
.if ${OS_NAME} == "SunOS"
CFLAGS+=    -I/usr/include -D__EXTENSIONS__ -include alloca.h
CPPFLAGS+=  -I/usr/include
LDFLAGS+=   -L/usr/lib/64 -L/usr/gnu/lib
.export CFLAGS CPPFLAGS LDFLAGS
.endif
