export interface IVector2 {
    readonly x: number;
    readonly y: number;

    set(x: number, y: number): IVector2;
    add(other: IVector2): IVector2;
    toAdded(other: IVector2): IVector2;
    subtract(other: IVector2): IVector2;
    toSubtracted(other: IVector2): IVector2;
    scale(scaler: number): IVector2;
    toScaled(scaler: number): IVector2;
    divide(divider: number): IVector2;
    toDivided(divider: number): IVector2;
    getMagnitude(): number;
    getSquaredMagnitude(): number;
    getDotProduct(other: IVector2): number;
    getCrossProduct(other: IVector2): number;
    normalize(): IVector2;
    toNormalized(): IVector2;
    createPerpendicular(): IVector2;
    rotate(angle: number): IVector2;
    toRotated(angle: number): IVector2;
    rotateAt(center: IVector2, angle: number): IVector2;
    toRotatedAt(center: IVector2, angle: number): IVector2;
    clone(): IVector2;
    reset(): void;
}
