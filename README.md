### Key-value база данных

## Задание

Cервер слушает запросы на указанном IP:PORT и предоставляет API для
простой key-value базы данных.  Внутреннее хранилище сервера -
key:value-таблица.  Ключ имеет тип - Строка (максимальная длина 1024
символа).  Значение имеет тип - Строка (максимальная длина 1024 * 1024
символа).

База данных поддерживает операции:
    INSERT - добавить key:value;
    UPDATE - изменить key:value;
    DELETE - удалить key;
    GET - получить value по key.

Если ключ уже существует, то при операции INSERT База Данных возвращает ошибку, что запись не была добавлена.
Если ключ не существует, то при операции UPDATE База Данных возвращает ошибку, что запись отсутсвует.
Если ключ существует и значение совпадает, то при операции UPDATE База Данных возвращает ошибку, что значение не было изменено.
Если ключ не существует, то при операции DELETE База Данных возвращает ошибку об отсутствующей записи.
Если ключ не существует, то при операции GET База Данных возвращает ошибку об отсутствующей записи.

Клиент получает из командной строки:
   - адрес сервера;
   - команду;
   - ключ;
   - значение.

После выполнения команды клиент возвращает успешность выполнения и ошибку, если она возникла.

Сервер ведет статистику отправленных и полученных команд.
С периодичностью в 60 секунд, сервер выводит на std::cerr статистику:
    - количество записей в БД;
    - количество успешных/неуспешных операций INSERT;
    - количество успешных/неуспешных операций UPDATE;
    - количество успешных/неуспешных операций DELETE;
    - количество успешных/неуспешных операций GET.

## Реализация

* Rust version 1.51.0 (2fd73fabe 2021-03-23)
* tonic 0.4.2 -- gRPC
* tokio 1.5.0 -- асинхронность
* structopt 0.3.21 -- интерфейс командной строки

## Тестирование

* Юнит-тесты как обычно: cargo test --release

* Скрипт smoke-test.sh проверяет готовность программ к запуску

* Скрипт integration-test.sh исполняет некоторые сценарии работы с проверкой результатов

* Скрипт stress-test.sh подвергает сервер повышенной нагрузке и оценивает производительность (не реализовано)
