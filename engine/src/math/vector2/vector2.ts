import type { IVector2 } from "./ivector2";

export class Vector2 implements IVector2 {
    public x: number;
    public y: number;

    public constructor();
    public constructor(x: number, y: number);
    public constructor(x = 0, y = 0) {
        this.x = x;
        this.y = y;
    }

    public set(x: number, y: number): IVector2 {
        this.x = x;
        this.y = y;

        return this;
    }

    public add(other: IVector2): IVector2 {
        this.x += other.x;
        this.y += other.y;

        return this;
    }

    public toAdded(other: IVector2): IVector2 {
        return this.clone().add(other);
    }

    public subtract(other: IVector2): IVector2 {
        this.x -= other.x;
        this.y -= other.y;

        return this;
    }

    public toSubtracted(other: IVector2): IVector2 {
        return this.clone().subtract(other);
    }

    public scale(scaler: number): IVector2 {
        this.x *= scaler;
        this.y *= scaler;

        return this;
    }

    public toScaled(scaler: number): IVector2 {
        return this.clone().scale(scaler);
    }

    public divide(divider: number): IVector2 {
        this.x /= divider;
        this.y /= divider;

        return this;
    }

    public toDivided(divider: number): IVector2 {
        return this.clone().divide(divider);
    }

    public getMagnitude(): number {
        return Math.sqrt(this.getSquaredMagnitude());
    }

    public getSquaredMagnitude(): number {
        return this.x ** 2 + this.y ** 2;
    }

    public getDotProduct(other: IVector2): number {
        return this.x * other.x + this.y * other.y;
    }

    public getCrossProduct(other: IVector2): number {
        return this.x * other.y - this.y * other.x;
    }

    public normalize(): IVector2 {
        const magnitude = this.getMagnitude();

        if (magnitude === 0) {
            return this;
        }

        this.x /= magnitude;
        this.y /= magnitude;

        return this;
    }

    public toNormalized(): IVector2 {
        return this.clone().normalize();
    }

    public createPerpendicular(): IVector2 {
        return new Vector2(this.y, -this.x).toNormalized();
    }

    public rotate(angle: number): IVector2 {
        const cos = Math.cos(angle);
        const sin = Math.sin(angle);

        const x = this.x * cos - this.y * sin;
        const y = this.x * sin + this.y * cos;

        this.x = x;
        this.y = y;

        return this;
    }

    public toRotated(angle: number): IVector2 {
        return this.clone().rotate(angle);
    }

    public rotateAt(center: IVector2, angle: number): IVector2 {
        const x = this.x - center.x;
        const y = this.y - center.y;

        const temporaryVector = new Vector2(x, y);

        temporaryVector.rotate(angle);

        temporaryVector.x += center.x;
        temporaryVector.y += center.y;

        this.x = temporaryVector.x;
        this.y = temporaryVector.y;

        return this;
    }

    public toRotatedAt(center: IVector2, angle: number): IVector2 {
        return this.clone().rotateAt(center, angle);
    }

    public clone(): IVector2 {
        return new Vector2(this.x, this.y);
    }

    public reset(): void {
        this.x = 0;
        this.y = 0;
    }
}
