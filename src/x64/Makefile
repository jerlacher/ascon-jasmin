RR=cargo
RDIR=rust
RLIBDIR=${RDIR}/src/rlib

JJ=jasminc
JECLIB=${HOME}/Software/jasmin/eclib

EC=easycrypt
EDIR=proof

test-rust: gen-rlib
	(cd $(RDIR) && cargo test)

bench-rust: gen-rlib
	(cd $(RDIR) && cargo criterion)

run-rust: gen-rlib
	(cd $(RDIR) && cargo run)

gen-rlib: compile
	(cd $(RLIBDIR) && ar -crs libascon.rlib ascon_128.o ascon_128_a.o lib.rmeta)

compile: compile-jazz
	$(CC) -c ascon_128.s -o ${RLIBDIR}/ascon_128.o
	$(CC) -c ascon_128_a.s -o ${RLIBDIR}/ascon_128_a.o

compile-jazz: ascon_128.s ascon_128_a.s

ascon_128.s: ascon_128.jazz jasmin/*.jinc
	$(JJ) -o $@ ascon_128.jazz

ascon_128_a.s: ascon_128_a.jazz jasmin/*.jinc
	$(JJ) -o $@ ascon_128_a.jazz

proof-ct: clean-ec
	$(JJ) -CT -oec $(EDIR)/ascon128_ct.ec -oecarray $(EDIR) ascon_128.jazz
	$(JJ) -CT -oec $(EDIR)/ascon128a_ct.ec -oecarray $(EDIR) ascon_128_a.jazz
	(cd $(EDIR) && $(EC) ascon_ct_proof.ec)

proof-safety:
	$(JJ) -checksafety -safetyconfig range_config.json ascon_128.jazz -safetyparam "m,c,a,p;mlen,clen,alen"
	$(JJ) -checksafety -safetyconfig range_config.json ascon_128_a.jazz -safetyparam "m,c,a,p;mlen,clen,alen"

clean: clean-ec clean-rlib
	rm -f *.o *.s

clean-ec:
	rm -f $(EDIR)/Array*.ec $(EDIR)/WArray*.ec $(EDIR)/*.eco $(EDIR)/*.stats \
		$(EDIR)/ascon128_ct.ec $(EDIR)/ascon128a_ct.ec

clean-rlib:
	rm -f $(RLIBDIR)/*.o 
