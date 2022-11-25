JJ=jasminc

compile-jazz: ascon_128.s ascon_128_a.s

ascon_128.s: ascon_128.jazz jasmin/*.jinc
	$(JJ) -arch arm-m4 -o $@ ascon_128.jazz

ascon_128_a.s: ascon_128_a.jazz jasmin/*.jinc
	$(JJ) -arch arm-m4 -o $@ ascon_128_a.jazz

clean:
	rm -f *.o *.s
