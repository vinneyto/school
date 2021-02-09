CC=g++
INC_DIR = ../common
CFLAGS=-c -std=c++17 -Wall -pedantic -I$(INC_DIR)
LDFLAGS=
SOURCES=*.cpp
OBJECTS=$(SOURCES:.cpp=.o)
EXECUTABLE=renderer

run: all render

render:
	./$(EXECUTABLE)

all: $(SOURCES) $(EXECUTABLE)

$(EXECUTABLE): $(OBJECTS)
	$(CC) $(LDFLAGS) $(OBJECTS) -o $@

.cpp.o:
	$(CC) $(CFLAGS) $< -o $@
