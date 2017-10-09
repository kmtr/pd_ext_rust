PD_RESOURCE := ../libpd/pure-data
PD_SRC = $(PD_RESOURCE)/src
CFLAGS = -I$(PD_SRC) -fPIC -arch x86_64 -c
LFLAGS = -dynamic -bundle -undefined dynamic_lookup


.PHONY: all
all:
	$(MAKE) helloworld.pd_darwin

%.pd_darwin: %.o
	$(CC) $(LFLAGS) -o $@ $^

%.o: %.c
	$(CC) $(CFLAGS) -o $@ $^

.PHONY: clean
clean:
	rm -f *.pd_darwin
	rm -f *.o