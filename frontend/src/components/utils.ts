export type Result<T, E> = { Ok: T } | { Err: E };

export type PrimaryOrSecondary = 'primary' | 'secondary';

export function toggle_primary_secondary<T>(
    toggle: PrimaryOrSecondary,
    primary_option: T,
    secondary_option: T
): T {
    switch (toggle) {
        case 'primary':
            return primary_option;
        case 'secondary':
            return secondary_option;
    }
}