RUSTC = rustc
FLAGS =
SRC = $(wildcard src/*.rs)
CRATE = src/rustracer.rs
NAME = rustracer

RUST_PNG = lib/rust-png
RUST_PNG_DUMMY = $(RUST_PNG)/librustpng.dummy

DEPS = $(RUST_PNG)

$(NAME): $(SRC) $(DEPS)
	$(RUSTC) $(FLAGS) $(addprefix -L ,$(DEPS)) -o $(NAME) $(CRATE)

$(RUST_PNG): $(RUST_PNG_DUMMY)

$(RUST_PNG_DUMMY):
	cd $(RUST_PNG); ./configure; true
	make -C $(RUST_PNG)
