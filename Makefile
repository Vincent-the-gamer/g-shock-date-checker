# Use Makefile to compile
# Make sure your name is Makefile

# frontend directory
FRONTEND_DIR := crates/frontend/

# server directory
SERVER_DIR := crates/server/

usage_msg := "Usage: \n  make target=frontend:  compile frontend\n  make target=server:  compile server"

ifeq (${target},frontend)
	target_dir = ${FRONTEND_DIR}
	command = "trunk build --release"
else ifeq (${target},server)
	target_dir = ${SERVER_DIR}
	command = "cargo build --release"
else
	target_dir = err
endif

# build
build:
ifndef target
	@echo ${usage_msg}
else
ifneq (${target_dir},err)
	@cd ${target_dir} && ${command}
else
	@echo ${usage_msg}
endif
endif




