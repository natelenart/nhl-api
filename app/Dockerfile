FROM composer:latest AS composer

FROM php:7-cli-alpine

WORKDIR /app

RUN addgroup -S php && \
    adduser -S php -G php

RUN apk update && \
    apk add --no-cache --virtual .build-deps \
    curl \
    ca-certificates \
    && \
    docker-php-ext-install pdo pdo_mysql && \
    apk del .build-deps

COPY . .

USER php

COPY --from=composer /usr/bin/composer /usr/bin/composer
RUN composer global require --no-plugins --no-scripts hirak/prestissimo

CMD ["tail", "-f", "/dev/null"]