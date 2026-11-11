import * as cy from 'clunkyard';

export class Probe extends cy.Robot {
  onTurn(): void {
    const target: cy.Contact | null = this.sensors.nearest;
    const power: number = this.gun.powerForFlightTime(target ? target.range : 200, 1);
    const solution: cy.Solution | null = target ? this.gun.lead(target, power) : null;
    this.drive.set({ throttle: 0.4, turn: cy.angles.clamp(0.25) });
    this.scanner.set({ heading: solution ? solution.heading : this.me.heading, arc: 40 });
    this.status.set({ text: 'TYPE_CHECKED', value: this.clock.tick });
  }
}

// These deliberate errors verify useful public declarations are present.
function rejectedControls(robot: cy.Robot): void {
  // @ts-expect-error Facts are read-only observations.
  robot.me.health = 100;
  // @ts-expect-error Unknown control fields must not silently compile.
  robot.drive.set({ throtle: 0.5 });
  // @ts-expect-error Firing power is numeric.
  robot.gun.fire({ power: 'heavy' });
}
void rejectedControls;
