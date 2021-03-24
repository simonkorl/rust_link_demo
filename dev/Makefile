INC_PATH = ./include 
LIB_PATH = ./lib
OUT_INC_PATH = ../src/link_c/include
PLUGIN_LIB = $(LIB_PATH)/libouter.so
DTP_LIB = $(LIB_PATH)/liblink_test.a
INCS = -I. -I$(INC_PATH) -I$(OUT_INC_PATH)
C_FLAGS = -W -O1
CPP_FLAG = -lstdc++

run: build
	LD_LIBRARY_PATH=./lib ./main

build: main.c ./include/api.h $(PLUGIN_LIB) $(DTP_LIB)
	gcc main.c $(C_FLAGS) -L$(LIB_PATH) -o main $(INCS) -llink_test -louter -ldl -pthread $(CPP_FLAG) 

$(DTP_LIB): ../target/release/liblink_test.a
	sudo mv ../target/release/liblink_test.a $@

$(PLUGIN_LIB): 
	cd plugin && make
	sudo mv ./plugin/libouter.so $@

../target/release/liblink_test.a: 
	cd .. && cargo build --release

.PHONY: 
	clean

clean:
	sudo rm ./lib/*
	sudo rm main 