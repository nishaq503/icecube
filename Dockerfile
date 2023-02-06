FROM kaggle/python:latest

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install maturin --locked

ENV CRATE_DIR="/icecube"
RUN mkdir "${CRATE_DIR}"
RUN mkdir ${CRATE_DIR}/src ${CRATE_DIR}/icecube
COPY pyproject.toml ${CRATE_DIR}/pyproject.toml
COPY Cargo.toml ${CRATE_DIR}/Cargo.toml
COPY src ${CRATE_DIR}/src
COPY icecube ${CRATE_DIR}/icecube
RUN rm -rf ${CRATE_DIR}/icecube/__pycache__
RUN rm -rf ${CRATE_DIR}/icecube/*.so
RUN cd ${CRATE_DIR} && maturin build --release --strip && cd ..
