package org.tby.fourdk.security.sample.spring.usecases.events;

import org.springframework.stereotype.Service;
import org.tby.fourdk.core.event.EventHandler;
import org.tby.fourdk.security.sample.spring.domain.GateRepository;
import org.tby.fourdk.security.sample.spring.domain.ParkingRepository;

@Service
public class OpenGateWhenANewCarEnteredEventHandler implements EventHandler<ANewCarIsParkedEvent> {

    private final ParkingRepository parkingRepository;

    private final GateRepository gateRepository;

    public OpenGateWhenANewCarEnteredEventHandler(ParkingRepository parkingRepository, GateRepository gateRepository) {
        this.parkingRepository = parkingRepository;
        this.gateRepository = gateRepository;
    }

    @Override
    public void handle(ANewCarIsParkedEvent event) {
        var parkingOpt = this.parkingRepository.findById(event.parkingId);
        if (parkingOpt.isPresent()) {
            var parking = parkingOpt.get();
            this.gateRepository.openTheGate(parking.getGateId());
        }
    }

    @Override
    public Class<ANewCarIsParkedEvent> getEventType() {
        return ANewCarIsParkedEvent.class;
    }
}
