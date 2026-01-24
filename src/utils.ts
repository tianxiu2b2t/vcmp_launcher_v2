import { Duration, Server } from './types';

export function hashServer(server: Server): string {
    return `${server.ip}-${server.port}`;
}

export function toMillis(duration: Duration): number {
    return duration.secs * 1000.0 + duration.nanos / 1e6;
}
