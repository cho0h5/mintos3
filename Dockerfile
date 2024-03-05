FROM ubuntu:22.04

RUN apt update && apt upgrade -y
RUN apt install -y \
	qemu \
	qemu-system-x86 \
	build-essential \
	git \
	nasm \
	wget \
	tmux \
	zsh \
	curl \
  cmake \
  gettext \
  clangd \
  unzip

RUN chsh -s /usr/bin/zsh

RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

RUN git clone https://github.com/LazyVim/starter ~/.config/nvim && \
	rm -rf ~/.config/nvim/.git

CMD ["/usr/bin/zsh"]

