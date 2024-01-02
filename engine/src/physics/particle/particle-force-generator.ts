import { type IVector2, Vector2 } from "../../math/vector2";
import { type IParticlePhysics } from "./iparticle-physics";
import { clamp } from "../../utils/clamp";

export class ParticleForceGenerator {
    private constructor() {}

    public static weightForce(particle: IParticlePhysics): Vector2 {
        const mass = particle.getMass();

        if (mass === 0) {
            return new Vector2();
        }

        return particle.getGravity().toScaled(mass);
    }

    public static dragForce(particle: IParticlePhysics, constant: number): IVector2 {
        if (particle.getMass() === 0) {
            return new Vector2();
        }

        const velocitySquaredMagnitude = particle.getVelocity().getSquaredMagnitude();

        if (velocitySquaredMagnitude <= 0) {
            return new Vector2();
        }

        const dragDirection = particle.getVelocity().toNormalized().scale(-1);
        const dragMagnitude = constant * velocitySquaredMagnitude;

        return dragDirection.scale(dragMagnitude);
    }

    public static frictionForce(particle: IParticlePhysics, constant: number): IVector2 {
        if (particle.getMass() === 0) {
            return new Vector2();
        }

        const frictionDirection = particle.getVelocity().toNormalized().scale(-1);

        return frictionDirection.scale(constant);
    }

    public static gravitationForce(
        particle: IParticlePhysics, other: IParticlePhysics,
        gravity: number,
        minDistance: number, maxDistance: number,
    ): IVector2 {
        if (particle.getMass() === 0) {
            return new Vector2();
        }

        const distance = other.getPosition().toSubtracted(particle.getPosition());

        const squaredMagnitude = clamp(
            distance.getSquaredMagnitude(),
            minDistance,
            maxDistance,
        );

        const attractionDirection = distance.toNormalized();

        const attractionMagnitude = gravity * (particle.getMass() * other.getMass()) / squaredMagnitude;

        return attractionDirection.scale(attractionMagnitude);
    }

    public static springForce(
        particle: IParticlePhysics, anchor: IVector2,
        constant: number,
        restLength: number,
    ): IVector2;
    public static springForce(
        particle: IParticlePhysics, other: IParticlePhysics,
        constant: number,
        restLength: number,
    ): IVector2;
    public static springForce(
        particle: IParticlePhysics, other: IVector2 | IParticlePhysics,
        constant: number,
        restLength: number,
    ): IVector2 {
        const otherPosition: IVector2 = isVector2(other) ? other : other.getPosition();

        const d = particle.getPosition().toSubtracted(otherPosition);

        const displacement = d.getMagnitude() - restLength;

        const springDirection = d.toNormalized();

        const springMagnitude = -constant * displacement;

        return springDirection.scale(springMagnitude);
    }
}

function isVector2(value: unknown | IVector2): value is IVector2 {
    return value instanceof Vector2;
}
