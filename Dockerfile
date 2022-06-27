FROM  alpine:latest

VOLUME /opt/app/data

WORKDIR /opt/app

COPY ./target/release/craiyon-discord ./

CMD [ "craiyon-discord" ]