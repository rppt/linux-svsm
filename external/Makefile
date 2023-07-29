.PHONY: libm libcrt libssl libtpm libplatform
all: libtpm # libm libcrt libssl

LIBM_DIR = $(CURDIR)/libm
LIBCRT_DIR = $(CURDIR)/libcrt
LIBSSL_DIR = $(CURDIR)/wolfssl
LIBTPM_DIR = $(CURDIR)/ms-tpm-20-ref/TPMCmd
INST_DIR = $(CURDIR)/build

libm libcrt: create_dirs
	make PREFIX=$(INST_DIR)/lib/ -C $@ install

libssl: libm libcrt
	make -C $(LIBSSL_DIR)
	make -C $(LIBSSL_DIR) install

libtpm: libssl libm libcrt
	make -C $(LIBTPM_DIR) Platform/src/libplatform.a
	make -C $(LIBTPM_DIR) tpm/src/libtpm.a
	cp $(LIBTPM_DIR)/Platform/src/libplatform.a $(INST_DIR)/lib
	cp $(LIBTPM_DIR)/tpm/src/libtpm.a $(INST_DIR)/lib

.PHONY: prereq
#prereq: libssl_config libtpm_config
prereq: libtpm_config

create_dirs:
	mkdir -p $(INST_DIR)/lib

LIBSSL_INCLUDES =  -I$(LIBSSL_DIR)/amd-svsm
LIBSSL_INCLUDES += -I$(LIBM_DIR)/include -I$(LIBCRT_DIR)/include

LIBSSL_CFLAGS =	 -fno-stack-protector -fPIE -nostdinc -Wno-error=float-equal
LIBSSL_CFLAGS += $(LIBSSL_INCLUDES)

LIBSSL_LDFLAGS = -L$(INST_DIR)/lib/ -lcrt -lm

LIBSSL_CONFIG_FLAGS = --enable-static --disable-shared --enable-usersettings --host=x86_64  --disable-crypttests --disable-examples --prefix=$(INST_DIR) --libdir=$(INST_DIR)/lib

.PHONY: libssl_config
libssl_config: libm libcrt
	cd $(LIBSSL_DIR) && \
	./autogen.sh && \
	CFLAGS="$(LIBSSL_CFLAGS)" LDFLAGS="$(LIBSSL_LDFLAGS)" ./configure $(LIBSSL_CONFIG_FLAGS)

# ./configure --enable-static --disable-shared --prefix=/home/mike/git/svsm-vtpm/linux-svsm/external/build --with-crypto-engine=Wolf CFLAGS="-fno-stack-protector -DSIMULATION=NO -DEPHEMERAL_NV -DVTPM -DWOLF_USER_SETTINGS -I/home/mike/git/svsm-vtpm/linux-svsm/external/wolfssl/amd-svsm -I/home/mike/git/svsm-vtpm/linux-svsm/external/libcrt/include -I/home/mike/git/svsm-vtpm/linux-svsm/external/libm/include -I/home/mike/git/svsm-vtpm/linux-svsm/external/build/include" LIBCRYPTO_CFLAGS="-I/home/mike/git/svsm-vtpm/linux-svsm/external/build/include/wolfssl" LIBCRYPTO_LIBS=/home/mike/git/svsm-vtpm/linux-svsm/external/build/lib/libwolfssl.a LIBS="/home/mike/git/svsm-vtpm/linux-svsm/external/build/lib/libcrt.a /home/mike/git/svsm-vtpm/linux-svsm/external/build/lib/libm.a"

LIBCRYPTO_CFLAGS = -I$(INST_DIR)/include/wolfssl
LIBCRYPTO_LIBS=$(INST_DIR)/lib/libwolfssl.a

LIBTPM_INCLUDES = -I$(LIBSSL_DIR)/amd-svsm -I$(LIBCRT_DIR)/include -I$(LIBM_DIR)/include -I$(INST_DIR)/include
LIBTPM_DEFINES = -DSIMULATION=NO -DEPHEMERAL_NV -DVTPM -DWOLF_USER_SETTINGS
LIBTPM_CFLAGS = -fno-stack-protector $(LIBTPM_DEFINES) $(LIBTPM_INCLUDES)
LIBTPM_LIBS = $(INST_DIR)/lib/libcrt.a $(INST_DIR)/lib/libm.a

LIBTPM_CONFIG_FLAGS = --prefix=$(INST_DIR) --with-crypto-engine=Wolf

.PHONY: libtpm_config
libtpm_config: libssl libm libcrt
	cd $(LIBTPM_DIR) && \
	./bootstrap && \
	CFLAGS="$(LIBTPM_CFLAGS)" LIBCRYPTO_LIBS="$(LIBCRYPTO_LIBS)" LIBS="$(LIBTPM_LIBS)" ./configure $(LIBTPM_CONFIG_FLAGS)
