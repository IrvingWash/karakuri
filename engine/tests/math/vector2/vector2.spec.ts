import { describe, expect, it } from "@jest/globals";

import { Vector2, type IVector2 } from "../../../src/math/vector2";

const NINETY_DEGREES_IN_RADIANS = 1.5708;

describe("Vector2", () => {
    it("should create a zero vector if no parameters were passed", () => {
        const v: IVector2 = new Vector2();

        expect(v.x).toEqual(0);
        expect(v.y).toEqual(0);
    });

    it("should create a vector of the passed values", () => {
        const x = -23.2301;
        const y = 5200;

        const v: IVector2 = new Vector2(x, y);

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);
    });

    it("should add another vector", () => {
        const x1 = 5;
        const y1 = -2;
        const x2 = 30;
        const y2 = 0.23;

        const v1: IVector2 = new Vector2(x1, y1);
        const v2: IVector2 = new Vector2(x2, y2);

        const v1Ref = v1.add(v2);

        expect(v1.x).toEqual(x1 + x2);
        expect(v1.y).toEqual(y1 + y2);

        expect(v1Ref).toBe(v1);
        expect(v1Ref).not.toBe(v2);

        expect(v2.x).toEqual(x2);
        expect(v2.y).toEqual(y2);
    });

    it("should create an added clone", () => {
        const x1 = 5;
        const y1 = -2;
        const x2 = 30;
        const y2 = 0.23;

        const v1: IVector2 = new Vector2(x1, y1);
        const v2: IVector2 = new Vector2(x2, y2);

        const vAdded = v1.toAdded(v2);

        expect(vAdded.x).toEqual(x1 + x2);
        expect(vAdded.y).toEqual(y1 + y2);

        expect(vAdded).not.toBe(v1);
        expect(vAdded).not.toBe(v2);

        expect(v1.x).toEqual(x1);
        expect(v1.y).toEqual(y1);
        expect(v2.x).toEqual(x2);
        expect(v2.y).toEqual(y2);
    });

    it("should subtract another vector", () => {
        const x1 = 5;
        const y1 = -2;
        const x2 = 30;
        const y2 = 0.23;

        const v1: IVector2 = new Vector2(x1, y1);
        const v2: IVector2 = new Vector2(x2, y2);

        const v1Ref = v1.subtract(v2);

        expect(v1.x).toEqual(x1 - x2);
        expect(v1.y).toEqual(y1 - y2);

        expect(v1Ref).toBe(v1);
        expect(v1Ref).not.toBe(v2);

        expect(v2.x).toEqual(x2);
        expect(v2.y).toEqual(y2);
    });

    it("should create a subtracted clone", () => {
        const x1 = 5;
        const y1 = -2;
        const x2 = 30;
        const y2 = 0.23;

        const v1: IVector2 = new Vector2(x1, y1);
        const v2: IVector2 = new Vector2(x2, y2);

        const vSubtracted = v1.toSubtracted(v2);

        expect(vSubtracted.x).toEqual(x1 - x2);
        expect(vSubtracted.y).toEqual(y1 - y2);

        expect(vSubtracted).not.toBe(v1);
        expect(vSubtracted).not.toBe(v2);

        expect(v1.x).toEqual(x1);
        expect(v1.y).toEqual(y1);
        expect(v2.x).toEqual(x2);
        expect(v2.y).toEqual(y2);
    });

    it("should scale", () => {
        const x = -23.2301;
        const y = 5200;
        const scaler = 0.5;

        const v: IVector2 = new Vector2(x, y);

        const vRef = v.scale(scaler);

        expect(v.x).toEqual(x * scaler);
        expect(v.y).toEqual(y * scaler);

        expect(vRef).toBe(v);
    });

    it("should create a scaled clone", () => {
        const x = -23.2301;
        const y = 5200;
        const scaler = 0.5;

        const v: IVector2 = new Vector2(x, y);

        const vScaled = v.toScaled(scaler);

        expect(vScaled.x).toEqual(x * scaler);
        expect(vScaled.y).toEqual(y * scaler);

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);

        expect(vScaled).not.toBe(v);
    });

    it("should divide", () => {
        const x = -23.2301;
        const y = 5200;
        const divider = 0.5;

        const v: IVector2 = new Vector2(x, y);

        const vRef = v.divide(divider);

        expect(v.x).toEqual(x / divider);
        expect(v.y).toEqual(y / divider);

        expect(vRef).toBe(v);
    });

    it("should create a divided clone", () => {
        const x = -23.2301;
        const y = 5200;
        const divider = 0.5;

        const v: IVector2 = new Vector2(x, y);

        const vDivided = v.toDivided(divider);

        expect(vDivided.x).toEqual(x / divider);
        expect(vDivided.y).toEqual(y / divider);

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);

        expect(vDivided).not.toBe(v);
    });

    it("should return magnitude", () => {
        const x = 2;
        const y = -34.4;

        const v = new Vector2(x, y);

        const magnitude = v.getMagnitude();

        expect(magnitude).toEqual(34.458090486850836);

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);
    });

    it("should return squared magnitude", () => {
        const x = 2;
        const y = -34.4;

        const v = new Vector2(x, y);

        expect(v.getSquaredMagnitude()).toEqual(1187.36);

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);
    });

    it("should return dot product", () => {
        const x1 = 432;
        const y1 = 0;
        const x2 = -32;
        const y2 = -99;

        const v1 = new Vector2(x1, y1);
        const v2 = new Vector2(x2, y2);

        const dotProduct = v1.getDotProduct(v2);

        expect(dotProduct).toEqual(-13824);

        expect(v1.x).toEqual(x1);
        expect(v1.y).toEqual(y1);
        expect(v2.x).toEqual(x2);
        expect(v2.y).toEqual(y2);
    });

    it("should return cross product", () => {
        const x1 = 432;
        const y1 = 0;
        const x2 = -32;
        const y2 = -99;

        const v1 = new Vector2(x1, y1);
        const v2 = new Vector2(x2, y2);

        const crossProduct = v1.getCrossProduct(v2);

        expect(crossProduct).toEqual(-42768);

        expect(v1.x).toEqual(x1);
        expect(v1.y).toEqual(y1);
        expect(v2.x).toEqual(x2);
        expect(v2.y).toEqual(y2);
    });

    it("should normalize", () => {
        const v = new Vector2(-234, 309);

        const vRef = v.normalize();

        expect(v.x).toEqual(-0.6037086604052452);
        expect(v.y).toEqual(0.7972050259197468);

        expect(vRef).toBe(v);

        const zeroVector = new Vector2();

        zeroVector.normalize();

        expect(zeroVector.x).toEqual(0);
        expect(zeroVector.y).toEqual(0);
    });

    it("should return a normalized clone", () => {
        const x = -234;
        const y = 309;

        const v = new Vector2(x, y);

        const normalizedVector = v.toNormalized();

        expect(normalizedVector.x).toEqual(-0.6037086604052452);
        expect(normalizedVector.y).toEqual(0.7972050259197468);

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);
        expect(normalizedVector).not.toBe(v);
    });

    it("should create a perpendicular", () => {
        const x = -102.23;
        const y = 34;

        const v = new Vector2(x, y);

        const perpendicular = v.createPerpendicular();

        expect(perpendicular.x).toEqual(0.3155872375021863);
        expect(perpendicular.y).toEqual(0.9488965673484855);

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);
        expect(perpendicular).not.toBe(v);
    });

    it("should rotate", () => {
        const x = 34.343;
        const y = -27;

        const v = new Vector2(x, y);

        const vRef = v.rotate(NINETY_DEGREES_IN_RADIANS);

        expect(Math.round(v.x)).toEqual(-y);
        expect(v.y.toFixed(3)).toEqual(x.toString());

        expect(vRef).toBe(v);
    });

    it("should create a rotated clone", () => {
        const x = 34.343;
        const y = -27;

        const v = new Vector2(x, y);

        const counterClockwiseRotatedVector = v.toRotated(
            NINETY_DEGREES_IN_RADIANS,
        );

        expect(Math.round(counterClockwiseRotatedVector.x)).toEqual(-y);
        expect(counterClockwiseRotatedVector.y.toFixed(3)).toEqual(x.toString());

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);
        expect(counterClockwiseRotatedVector).not.toBe(v);

        const clockwiseRotatedVector = v.toRotated(-NINETY_DEGREES_IN_RADIANS);
        expect(Math.round(clockwiseRotatedVector.x)).toEqual(y);
        expect(clockwiseRotatedVector.y.toFixed(3)).toEqual((-x).toString());

        expect(v.x).toEqual(x);
        expect(v.y).toEqual(y);
        expect(clockwiseRotatedVector).not.toBe(v);
    });

    it("should rotate at specified axis", () => {
        const x1 = 34;
        const y1 = -27;
        const x2 = x1 * 0.5;
        const y2 = y1 * 0.5;

        const v = new Vector2(x1, y1);
        const axis = new Vector2(x2, y2);

        const vRef = v.rotateAt(axis, NINETY_DEGREES_IN_RADIANS);

        expect(v.x).toEqual(30.49993755542217);
        expect(v.y).toEqual(3.5000495881542086);

        expect(vRef).toBe(v);
        expect(axis).not.toBe(v);

        expect(axis).toEqual(new Vector2(x2, y2));
    });

    it("should return a clone rotated at specified axis", () => {
        const x1 = 34;
        const y1 = -27;
        const x2 = x1 * 0.5;
        const y2 = y1 * 0.5;

        const v = new Vector2(x1, y1);
        const axis = new Vector2(x2, y2);

        const rotatedAtAxisVector = v.toRotatedAt(axis, NINETY_DEGREES_IN_RADIANS);

        expect(rotatedAtAxisVector.x).toEqual(30.49993755542217);
        expect(rotatedAtAxisVector.y).toEqual(3.5000495881542086);

        expect(rotatedAtAxisVector).not.toBe(v);
        expect(rotatedAtAxisVector).not.toBe(axis);

        expect(v).toEqual(new Vector2(x1, y1));
        expect(axis).toEqual(new Vector2(x2, y2));
    });

    it("should create a clone", () => {
        const x = -23.2301;
        const y = 5200;

        const v: IVector2 = new Vector2(x, y);
        const vClone = v.clone();

        expect(vClone).toEqual(v);
        expect(vClone).not.toBe(v);
    });

    it("should reset", () => {
        const v: IVector2 = new Vector2(984, -232.03);

        v.reset();

        expect(v).toEqual(new Vector2(0, 0));
    });

    it("should set", () => {
        const v: IVector2 = new Vector2(3, 3);

        v.set(-1, 0);

        expect(v).toEqual(new Vector2(-1, 0));
    });
});
