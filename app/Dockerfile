FROM composer:latest AS composer

FROM php:7.3-fpm-alpine AS base-app
ARG TIMEZONE

WORKDIR /app

# Set timezone appropriately
RUN ln -snf /usr/share/zoneinfo/${TIMEZONE} /etc/localtime \
    && echo ${TIMEZONE} > /etc/timezone \
    && printf '[PHP]\ndate.timezone = "%s"\n', ${TIMEZONE} > /usr/local/etc/php/conf.d/tzone.ini \
    && php -r "echo date_format(date_create('now'), 'c') . PHP_EOL;"

# Install needed server/PHP dependencies
RUN apk update && \
    apk add --no-cache --virtual .build-deps \
    curl \
    ca-certificates \
    && \
    docker-php-ext-install pdo pdo_mysql && \
    apk del .build-deps

# Make sure the non-privileged user owns the working dir
RUN chown www-data .

# Don't run container as root
USER www-data

# Pull latest composer and require prestissimo for parallel processing
COPY --from=composer /usr/bin/composer /usr/bin/composer
RUN composer global require --no-plugins --no-scripts hirak/prestissimo


# Production build
FROM base-app AS nhl-app

# Install vendor libs only (allows intermediate caching)
COPY --chown=www-data composer.json composer.lock ./
RUN composer install \
    --prefer-dist \
    --no-dev \
    --no-autoloader \
    --no-scripts \
    --no-suggest \
    --no-plugins

# Copy source files
COPY --chown=www-data . .

# Dump the autoload files we skipped earlier in an automated fashion
RUN composer dump-autoload \
    --no-dev \
    --classmap-authoritative


# Dev build
FROM nhl-app AS nhl-app-dev

# Add in Nginx
# ARG XDEBUG_VERSION=2.6.0
# RUN set -eux; \
# 	apk add --no-cache --virtual .build-deps $PHPIZE_DEPS; \
# 	pecl install xdebug-$XDEBUG_VERSION; \
# 	docker-php-ext-enable xdebug; \
# 	apk del .build-deps

# Install dev libraries
RUN composer install \
    --prefer-dist \
    --no-autoloader \
    --no-scripts \
    --no-suggest \
    --no-plugins \
    && composer dump-autoload \
    --classmap-authoritative