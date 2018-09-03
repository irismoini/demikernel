CC = g++

ZEUS_SRC_DIR=$(HOME)/datacenter-OS/
LIBZEUS=zeus_posix

ZEUS_CFLAGS := -I$(ZEUS_SRC_DIR)
ZEUS_LDFLAGS := -L$(ZEUS_SRC_DIR) -l$(LIBZEUS)
MTCP_LDFLAGS := $(ZEUS_LDFLAGS)
MTCP_LDFLAGS += -L$(ZEUS_SRC_DIR)/libos/libmtcp/mtcp/dpdk/lib -L$(ZEUS_SRC_DIR)/libos/libmtcp/mtcp/mtcp/lib -lmtcp

DPDK_HOME=$(ZEUS_SRC_DIR)/libos/libmtcp/mtcp/dpdk/
DPDK_INC=$(DPDK_HOME)/include
DPDK_LIB=$(DPDK_HOME)/lib/
DPDK_MACHINE_FLAGS = $(shell cat $(HOME)/datacenter-OS/libos/libmtcp/mtcp/dpdk/include/cflags.txt)
DPDK_LIB_FLAGS = $(shell cat $(HOME)/datacenter-OS/libos/libmtcp/mtcp/dpdk/lib/ldflags.txt) -lgmp

# MTCP
MTCP_FLD    =$(HOME)/datacenter-OS/libos/libmtcp/mtcp/mtcp/
MTCP_INC    =-I$(MTCP_FLD)/include
MTCP_LIB    =-L$(MTCP_FLD)/lib
MTCP_TARGET = $(MTCP_LIB)/lib/libmtcp.a

FINAL_CFLAGS=$(STD) $(WARN) $(OPT) $(CFLAGS) -O3 -DNDEBUG
FINAL_LDFLAGS=$(LDFLAGS)
FINAL_LIBS=-lm

ZEUS_LIBS := -l$(LIBZEUS) -lhoard -Wl,-rpath,$(ZEUS_SRC_DIR)
FINAL_CFLAGS += $(ZEUS_CFLAGS)
FINAL_LDFLAGS += $(ZEUS_LDFLAGS) -lhoard -Wl,-rpath,$(ZEUS_SRC_DIR)
FINAL_LIBS += $(ZEUS_LIBS)
FINAL_LIBS += -pthread -lrt -march=native -export-dynamic -lnuma -lpthread -lrt -ldl -lstdc++

CFLAGS_CXX=-std=c++0x

all:
	${CC} -o tcp_server tcp_server.cc ${FINAL_CFLAGS} ${CFLAGS_CXX} ${FINAL_LDFLAGS} ${FINAL_LIBS}