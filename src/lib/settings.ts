import { Store } from '@tauri-apps/plugin-store';

export type SpotifySettings = {
    enabled: boolean;
} & (
        { enabled: true; clientId: string; } |
        { enabled: false; clientId?: never; }
    );

// Define category interfaces
export interface AppearanceSettings {
    activeTheme: string;
}

export interface FeatureSettings {
    spotify: SpotifySettings;
}

// Update main schema
export interface SettingsSchema {
    appearance: AppearanceSettings;
    features: FeatureSettings;
}

// Define default values
export const DEFAULT_SETTINGS: SettingsSchema = {
    appearance: {
        activeTheme: 'coffee'
    },
    features: {
        spotify: {
            enabled: false
        }
    }
} as const;



export class SettingsManager {
    private settings: SettingsSchema;
    private store: Store;

    constructor(store: Store, initialSettings?: Partial<SettingsSchema>) {
        this.store = store;
        this.settings = {
            ...DEFAULT_SETTINGS,
            ...initialSettings
        };
    }

    async initialize(): Promise<void> {
        const stored = await this.store.get<SettingsSchema>('settings');
        if (stored) {
            this.settings = stored;
        } else {
            await this.store.set('settings', this.settings);
        }
    }

    // Single-level set
    async set<K extends keyof SettingsSchema>(
        key: K,
        value: SettingsSchema[K]
    ): Promise<void>;
    // Nested set
    async set<K extends keyof SettingsSchema, SK extends keyof SettingsSchema[K]>(
        key: K,
        subKey: SK,
        value: SettingsSchema[K][SK]
    ): Promise<void>;
    // Implementation
    async set<K extends keyof SettingsSchema, SK extends keyof SettingsSchema[K]>(
        key: K,
        subKeyOrValue: SK | SettingsSchema[K],
        value?: SettingsSchema[K][SK]
    ): Promise<void> {
        if (value !== undefined) {
            this.settings[key][subKeyOrValue as SK] = value;
        } else {
            this.settings[key] = subKeyOrValue as SettingsSchema[K];
        }
        await this.save();
    }

    async update(newSettings: Partial<SettingsSchema>): Promise<void> {
        this.settings = {
            ...this.settings,
            ...newSettings
        };
        await this.save();
    }

    private async save(): Promise<void> {
        await this.store.set('settings', this.settings);
    }

    // Single-level get
    get<K extends keyof SettingsSchema>(key: K): SettingsSchema[K];
    // Nested get
    get<K extends keyof SettingsSchema, SK extends keyof SettingsSchema[K]>(
        key: K,
        subKey: SK
    ): SettingsSchema[K][SK];
    // Implementation
    get<K extends keyof SettingsSchema, SK extends keyof SettingsSchema[K]>(
        key: K,
        subKey?: SK
    ): SettingsSchema[K] | SettingsSchema[K][SK] {
        if (subKey !== undefined) {
            return this.settings[key][subKey];
        }
        return this.settings[key];
    }

    getAll(): SettingsSchema {
        return { ...this.settings };
    }

    static async create(store: Store): Promise<SettingsManager> {
        const manager = new SettingsManager(store);
        await manager.initialize();
        return manager;
    }
}
