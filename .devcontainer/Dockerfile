ARG VARIANT=latest
FROM rcore-tutorial:${VARIANT}

# Install Fish shell
RUN apt-get update && apt-get install -y fish

# Set Fish as the default shell
ENV SHELL=/usr/bin/fish