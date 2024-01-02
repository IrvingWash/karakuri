import { describe, expect, it } from "@jest/globals";

import { IParticlePhysics, ParticleForceGenerator, ParticlePhysics } from "../../../src/physics/particle";
import { IVector2, Vector2 } from "../../../src/math/vector2";

describe("ParticleForceGenerator: Spring force", () => {
    const anchor = new ParticlePhysics({
        mass: 0,
        position: new Vector2(300, 0),
    });

    const ball = new ParticlePhysics({
        position: new Vector2(290, 50),
    });

    const force = ParticleForceGenerator.springForce(ball, anchor, 50, 200);

    expect(force).toEqual(new Vector2(-1461.1613513818404, 7305.806756909202));
});

describe("ParticleForceGenerator: Gravitation force", () => {
    const sunPosition = new Vector2(1000, 1000);

    it("should generate gravitation force", () => {
        const sun: IParticlePhysics = new ParticlePhysics({
            gravity: new Vector2(),
            mass: 1000,
            position: sunPosition.clone(),
        });

        const earth: IParticlePhysics = new ParticlePhysics({
            gravity: new Vector2(),
            mass: 1,
        });

        const force = ParticleForceGenerator.gravitationForce(earth, sun, 10, 100, 300);

        expect(force).toEqual(new Vector2(23.570226039551585, 23.570226039551585));
        expect(sun.getVelocity()).toEqual(new Vector2());
        expect(sun.getPosition()).toEqual(sunPosition.clone());
    });
});

describe("ParticleForceGenerator: Drag force", () => {
    it("should generate drag force", () => {
        const particle: IParticlePhysics = new ParticlePhysics({});

        particle.addForce(new Vector2(0, 100));
        particle.integrate(1);

        const dragForce = ParticleForceGenerator.dragForce(particle, 0.001);

        expect(dragForce).toEqual(new Vector2(-0, -10));
    });
});

describe("ParticleForceGenerator: Friction force", () => {
    it("should generate friction force", () => {
        const particle: IParticlePhysics = new ParticlePhysics({});

        particle.addForce(new Vector2(0, 100));
        particle.integrate(1);

        const dragForce = ParticleForceGenerator.dragForce(particle, 0.1);

        expect(dragForce).toEqual(new Vector2(-0, -1000));
    });
});

describe("ParticleForceGenerator: Weight force", () => {
    it("should generate downwards weight force", () => {
        const particleWithDownwardsGravity: IParticlePhysics = new ParticlePhysics({
            gravity: new Vector2(0, 10),
            mass: 10,
        });

        const downWardsForce = ParticleForceGenerator.weightForce(particleWithDownwardsGravity);

        expect(downWardsForce).toEqual(new Vector2(0, 100));
    });

    it("should generate upwards weight force", () => {
        const particleWithUpwardsGravity: IParticlePhysics = new ParticlePhysics({
            gravity: new Vector2(0, -10),
            mass: 10,
        });

        const upwardsForce = ParticleForceGenerator.weightForce(particleWithUpwardsGravity);

        expect(upwardsForce).toEqual(new Vector2(0, -100));
    });

    it("should generate rightwards weight force", () => {
        const particleWithRightwardsGravity: IParticlePhysics = new ParticlePhysics({
            gravity: new Vector2(10, 0),
            mass: 10,
        });

        const rightwardsForce = ParticleForceGenerator.weightForce(particleWithRightwardsGravity);

        expect(rightwardsForce).toEqual(new Vector2(100, 0));
    });

    it("should generate leftwards weight force", () => {
        const particleWithLeftwardsGravity: IParticlePhysics = new ParticlePhysics({
            gravity: new Vector2(-10, 0),
        });

        const leftwardsForce = ParticleForceGenerator.weightForce(particleWithLeftwardsGravity);

        expect(leftwardsForce).toEqual(new Vector2(-10, 0));
    });

    it("should vary depending on the mass", () => {
        let force: IVector2;

        const heavyParticle: IParticlePhysics = new ParticlePhysics({
            mass: 100,
        });
        force = ParticleForceGenerator.weightForce(heavyParticle);
        expect(force).toEqual(new Vector2(0, 980.0000000000001));

        const lightParticle: IParticlePhysics = new ParticlePhysics({
            mass: 0.01,
        });
        force = ParticleForceGenerator.weightForce(lightParticle);
        expect(force).toEqual(new Vector2(0, 0.098));
    });
});
