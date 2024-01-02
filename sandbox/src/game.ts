import {
    Karakuri,
    Vector2,
    Sprite,
    Transform,
    Particle,
    Behavior,
    ParticleForceGenerator,
    IVector2,
} from "karakuri";

import square from "../assets/square.png";
import circle from "../assets/circle.png";
import spring from "../assets/spring.png";

const PIXELS_PER_METER = 50;

class Ball extends Behavior {
    private _isHolding: boolean = false;
    private _mousePosition: IVector2 = new Vector2();

    public override onStart(): void {
        addEventListener("mousedown", () => {
            this._isHolding = true;
        });

        addEventListener("mouseup", () => {
            this._isHolding = false;
        });

        addEventListener("mousemove", (event) => {
            this._mousePosition = new Vector2(event.clientX, event.clientY);
        });
    }

    public override onUpdate(_deltaTime: number): void {
        if (this._isHolding) {
            this.transform.position.set(this._mousePosition.x, this._mousePosition.y);

            return;
        }

        const particle = this.particle?.getParticlePhysics();
        const anchorParticle = this.getEntity("anchor")?.particle?.getParticlePhysics();

        if (particle === undefined || anchorParticle === undefined) {
            return;
        }

        const springForce = ParticleForceGenerator.springForce(particle, anchorParticle, 100, 200);
        const dragForce = ParticleForceGenerator.dragForce(particle, 0.003);

        particle.addForce(springForce);
        particle.addForce(dragForce);
    }
}

class Rope extends Behavior {
    public override onUpdate(_deltaTime: number): void {
        const ball = this.getEntity("ball");
        const anchor = this.getEntity("anchor");

        if (ball === undefined || anchor === undefined) {
            return;
        }

        this.transform.position.set(anchor.transform.position.x, anchor.transform.position.y);
        this.transform.scale.set(0.1, ball.transform.position.y / 1000);
    }
}

export async function game(): Promise<void> {
    const engine = new Karakuri({ clearColor: [0.7, 0.7, 0.7, 1] });
    await engine.init();

    const level = engine.createScene();
    const canvasSize = engine.getCanvasSize();

    await level.createEntity({
        name: "rope",
        sprite: new Sprite({ path: spring }),
        behavior: new Rope(),
    });

    await level.createEntity({
        name: "anchor",
        particle: new Particle({ mass: 0 }),
        sprite: new Sprite({ path: square, color: [0, 1, 0, 1] }),
        transform: new Transform({
            position: new Vector2(canvasSize.width / 2, 0),
            scale: new Vector2(0.3, 0.3),
            rotation: new Vector2(45, 0),
        }),
    });

    await level.createEntity({
        name: "ball",
        particle: new Particle({ mass: 10, gravity: new Vector2(0, PIXELS_PER_METER * 10) }),
        sprite: new Sprite({ path: circle, color: [1, 0, 0, 1] }),
        transform: new Transform({
            position: new Vector2(canvasSize.width / 2, 50),
            scale: new Vector2(0.5, 0.5),
        }),
        behavior: new Ball(),
    });

    level.start();
}
