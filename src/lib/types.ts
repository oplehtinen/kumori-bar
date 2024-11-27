export interface KomorebiStatus {
    cross_monitor_move_behaviour: 'Insert' | 'Swap';
    focus_follows_mouse: boolean | null;
    has_pending_raise_op: boolean;
    is_paused: boolean;
    monitors: {
        elements: KomorebiMonitor[];
        focused: KomorebiMonitor[keyof KomorebiMonitor[]];
    };
    mouse_follows_focus: boolean;
    new_window_behaviour: 'Create' | 'Insert' | 'Swap';
    resize_delta: number;
    unmanaged_window_behaviour: 'Op' | 'Ignore';
    work_area_offset: number | null;
}
export interface KomorebiMonitor {
    id: number;
    deviceId: string;
    focusedWorkspaceIndex: number;
    name: string;
    size: KomorebiRect;
    workAreaOffset: number | null;
    workAreaSize: KomorebiRect;
    workspaces: {
        elements: KomorebiWorkspace[],
        focused: number
    };
}
export interface KomorebiWorkspace {
    containerPadding: number | null;
    floatingWindows: KomorebiWindow[];
    focusedContainerIndex: number;
    latestLayout: KomorebiRect[];
    layout: KomorebiLayout;
    layoutFlip: KomorebiLayoutFlip | null;
    maximizedWindow: KomorebiWindow | null;
    monocleContainer: KomorebiContainer | null;
    name: string | null;
    tilingContainers: KomorebiContainer[];
    workspacePadding: number | null;
}

export interface KomorebiContainer {
    id: string;
    windows: KomorebiWindow[];
}

export interface KomorebiWindow {
    class: string | null;
    exe: string | null;
    hwnd: number;
    title: string | null;
}

export interface KomorebiRect {
    left: number;
    top: number;
    right: number;
    bottom: number;
}

export type KomorebiLayout =
    | 'bsp'
    | 'vertical_stack'
    | 'horizontal_stack'
    | 'ultrawide_vertical_stack'
    | 'rows'
    | 'grid'
    | 'right_main_vertical_stack'
    | 'custom';

export type KomorebiLayoutFlip =
    | 'horizontal'
    | 'vertical'
    | 'horizontal_and_vertical';

