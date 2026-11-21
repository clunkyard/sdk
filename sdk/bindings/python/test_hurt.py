"""The hurt robot (sdk/bindings/hurt.js): names the cause that took the most
health, and reverses while the boundary hurts it."""
import clunkyard as cy


class Hurt(cy.Robot):
    def on_turn(self):
        hurt = self.me.hurt
        taken = [("SHOT", hurt.shot), ("BURST", hurt.burst), ("COLLISION", hurt.collision), ("BOUNDARY", hurt.boundary)]
        total, worst = 0.0, taken[0]
        for cause in taken:
            total += cause[1]
            if cause[1] > worst[1]:
                worst = cause
        self.status.set(worst[0] if total > 0 else "NONE", total)
        self.drive.set(throttle=-1 if hurt.boundary > 0 else 0)


cy.run(Hurt())
