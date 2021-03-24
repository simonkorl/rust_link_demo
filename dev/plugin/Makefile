INC_PATH = ../../src/link_c/include

share: outer.cxx $(INC_PATH)/outer.hxx
	g++ --share -fPIC outer.cxx -I$(INC_PATH) -lm -o libouter.so
	sudo cp ./libouter.so ../lib/libouter.so
